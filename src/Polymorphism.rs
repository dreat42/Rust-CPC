

use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address = address.convert_address().unwrap();
    converted_address
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_poly() {

         let addr_str = "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5";
        let addr: Address = Address::from_str(addr_str).unwrap();

        let new_addr :Address = get_ethereum_data(addr);

        assert_eq!(new_addr, Address::from_str(addr_str).unwrap());


        let new_addr :Address = get_ethereum_data(addr_str); 

        assert_eq!(new_addr, Address::from_str(addr_str).unwrap());


    }
}
