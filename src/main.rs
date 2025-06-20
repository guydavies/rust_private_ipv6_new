use rand::Rng;
use std::net::Ipv6Addr;

/// Generate a random hexadecimal string with a value up to the maximum provided
fn generate_hex(max_hex_value_as_int: u16) -> String {
  let random_value: u16 = rand::thread_rng().gen_range(0..=max_hex_value_as_int);
  if max_hex_value_as_int == 255 {
    return format!("{:02x}", random_value); // Ensure two digits for byte representation
  } else {
    return format!("{:0x}", random_value); // Return as the output digit
  }
}

fn create_unique_local_prefix() -> String {
  // Generate a random hexadecimal string for the unique local prefix
  let first_byte: &'static str = "fd"; // ULA prefixes are in fc00::/7 but fc00::/8 is reserved for future use
  let second_byte = format!("{}", generate_hex(255)); // Generate a random value for the second byte (0-255)
  let group1: String = format!("{}{}", first_byte, second_byte); // Combine the first byte with the second byte
  let group2 = format!("{}", generate_hex(65535)); // Generate a random hex value for the second 16 bit group (0-65535)
  let group3: String = format!("{}", generate_hex(65535)); // Generate a random value for the third 16 bit group (0-65535)
  let group4: String = format!("{}", generate_hex(65535)); // Generate a random value for the fourth 16 bit group (0-65535)

  let unique_local_prefix = format!("{}/64", Ipv6Addr::new(
    u16::from_str_radix(&group1, 16).unwrap(),
    u16::from_str_radix(&group2, 16).unwrap(),
    u16::from_str_radix(&group3, 16).unwrap(),
    u16::from_str_radix(&group4, 16).unwrap(),
    0,
    0,
    0,
    0,
    )
  );
  unique_local_prefix
}

fn main() {
  let unique_local_prefix = create_unique_local_prefix();
  println!("Generated Unique Local Address Prefix: {}", unique_local_prefix);
}
