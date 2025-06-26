use rand::Rng;
use std::net::Ipv6Addr;

/// Generate a random hexadecimal string with a value up to the maximum provided
fn generate_hex(max_hex_value_as_int: u16) -> String {
  let random_value: u16 = rand::rng().random_range(0..=max_hex_value_as_int);
  if max_hex_value_as_int == 255 {
    return format!("{:02x}", random_value); // Ensure two digits for byte representation
  } else {
    return format!("{:0x}", random_value); // Return as the output digit
  }
}

fn create_unique_local_prefix() -> (Ipv6Addr, String) {
  // Generate a random hexadecimal string for the unique local prefix
  let first_byte: &'static str = "fd"; // ULA prefixes are in fc00::/7 but fc00::/8 is reserved for future use
  let second_byte = format!("{}", generate_hex(255)); // Generate a random value for the second byte (0-255)
  let group1: String = format!("{}{}", first_byte, second_byte); // Combine the first byte with the second byte
  let group2 = format!("{}", generate_hex(65535)); // Generate a random hex value for the second 16 bit group (0-65535)
  let group3: String = format!("{}", generate_hex(65535)); // Generate a random value for the third 16 bit group (0-65535)
  let group4: String = format!("{}", generate_hex(65535)); // Generate a random value for the fourth 16 bit group (0-65535)

  let unique_local_addr: Ipv6Addr = Ipv6Addr::new(
    u16::from_str_radix(&group1, 16).unwrap(),
    u16::from_str_radix(&group2, 16).unwrap(),
    u16::from_str_radix(&group3, 16).unwrap(),
    u16::from_str_radix(&group4, 16).unwrap(),
    0,
    0,
    0,
    0,
    );

  let unique_local_prefix = format!("{}/64", unique_local_addr); // Append /64 for CIDR notation
  return (unique_local_addr, unique_local_prefix)
}

fn main() {
  let (_unique_local_addr, unique_local_prefix) = create_unique_local_prefix();
  println!("Generated Unique Local Address Prefix: {}", unique_local_prefix);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_generate_hex_with_max_255() {
        // Test with max value 255 (for byte representation)
        let result = generate_hex(255);

        // Verify the result is a valid hex string
        assert!(u8::from_str_radix(&result, 16).is_ok());

        // Verify the length is exactly 2 characters (ensuring two digits)
        assert_eq!(result.len(), 2);

        // Verify the result can be parsed as a valid u8 value
        let _value = u8::from_str_radix(&result, 16).unwrap();
    }

    #[test]
    fn test_generate_hex_with_max_65535() {
        // Test with max value 65535 (for 16-bit representation)
        let result = generate_hex(65535);

        // Verify the result is a valid hex string
        assert!(u16::from_str_radix(&result, 16).is_ok());

        // Verify the result can be parsed as a valid u16 value
        let _value = u16::from_str_radix(&result, 16).unwrap();
    }

    #[test]
    fn test_generate_hex_with_small_value() {
        // Test with a small max value
        let result = generate_hex(15);

        // Verify the result is a valid hex string
        assert!(u16::from_str_radix(&result, 16).is_ok());

        // Verify the value is within the expected range
        let value = u16::from_str_radix(&result, 16).unwrap();
        assert!(value <= 15);
    }

    #[test]
    fn test_generate_hex_with_zero() {
        // Test with max value 0
        let result = generate_hex(0);

        // Should always return "0"
        assert_eq!(result, "0");
    }

    #[test]
    fn test_create_unique_local_prefix_format() {
        // Test the format of the generated unique local prefix
        let (addr, prefix) = create_unique_local_prefix();

        // Verify it ends with "/64" (CIDR notation)
        assert!(prefix.ends_with("/64"));

        // Extract the IPv6 address part
        //let addr_part = prefix.split('/').next().unwrap();

        // Parse as IPv6 address
        //let addr = addr_part.parse::<Ipv6Addr>().unwrap();

        // Verify the first byte starts with fd (ULA prefix)
        let segments = addr.segments();
        assert_eq!(segments[0] >> 8, 0xfd);

        // Verify all zeros in the interface ID portion (last 64 bits)
        assert_eq!(segments[4], 0);
        assert_eq!(segments[5], 0);
        assert_eq!(segments[6], 0);
        assert_eq!(segments[7], 0);
    }

    #[test]
    fn test_create_unique_local_prefix_randomness() {
        // Test that multiple calls produce different prefixes
        let mut prefixes = HashSet::new();

        // Generate multiple prefixes
        for _ in 0..10 {
            let (_, prefix) = create_unique_local_prefix();
            prefixes.insert(prefix);
        }

        // If randomness works, we should have close to 10 unique values
        // (very small chance of collision)
        assert!(prefixes.len() >= 9);
    }

    #[test]
    fn test_create_unique_local_prefix_validity() {
        // Test that the generated prefix is a valid IPv6 ULA
        let (addr, _) = create_unique_local_prefix();

        // Extract the IPv6 address part
        //let addr_part = prefix.split('/').next().unwrap();

        // Parse as IPv6 address
        //let addr = addr_part.parse::<Ipv6Addr>().unwrap();

        // Check that it's a valid ULA (Unique Local Address)
        // ULA addresses start with fd00::/8
        let segments = addr.segments();
        assert_eq!(segments[0] & 0xff00, 0xfd00);

        // Verify the interface ID portion (last 64 bits) are zeros
        assert_eq!(segments[4], 0);
        assert_eq!(segments[5], 0);
        assert_eq!(segments[6], 0);
        assert_eq!(segments[7], 0);
    }

    #[test]
    fn test_generated_address_is_valid_ula() {
        // Test that the generated address is within the valid ULA range
        // fc00::/7 may be allocated for ULA, but fc00::/8 is reserved for future use
        // Only fd00::/8 is currently valid for ULA
        for _ in 0..100 {  // Test multiple generations to ensure consistency
            let (addr, _) = create_unique_local_prefix();

            // Get the first octet (most significant 8 bits)
            let first_octet = (addr.segments()[0] >> 8) as u8;

            // Ensure it's in the fd00::/8 range (0xfd)
            assert_eq!(first_octet, 0xfd,
                "Generated address {addr} is not in valid ULA range fd00::/8. First octet: 0x{first_octet:02x}");

            // Additional check: ensure it's NOT in the reserved fc00::/8 range
            assert_ne!(first_octet, 0xfc,
                "Generated address {addr} is in reserved fc00::/8 range, should be in fd00::/8");

            // Verify it's within the broader ULA range fc00::/7
            // (first bit of first octet should be 1 for fc00::/7)
            assert!(first_octet & 0xfe == 0xfc,
                "Generated address {addr} is not within ULA range fc00::/7");
        }
    }
}
