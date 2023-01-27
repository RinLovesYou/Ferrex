use bincode::{Encode, Decode, enc::write::Writer, de::read::Reader};

#[derive (Clone, Debug)]
pub struct ArbitraryData {
    pub data: Vec<Vec<u8>>,
}

impl Encode for ArbitraryData {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        self.data.len().encode(encoder)?;
        for d in self.data.iter() {
            d.len().encode(encoder)?;
            encoder.writer().write(d.as_slice());
        }

        Ok(())
    }
}

impl Decode for ArbitraryData {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let len = usize::decode(decoder)?;

        let mut data = Vec::new();

        for i in 0..len {
            let mut nested_data = Vec::new();
            let nested_len = usize::decode(decoder)?;
            nested_data.resize(nested_len, 0);
            
            decoder.reader().read(nested_data.as_mut_slice())?;
            data.push(nested_data);
        }

        Ok(ArbitraryData { 
            data 
        })
    }
}