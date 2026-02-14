




//
////use byteorder::{BigEndian, ByteOrder};
////
//use sqlx::{Postgres, Encode, Decode, Type};
//use sqlx::error::BoxDynError;
//use sqlx::encode::IsNull;
////
//use sqlx::postgres::{PgValueFormat, PgArgumentBuffer, PgTypeInfo};
////
////// I have no idea if this works. It will probably overflow
////
////
//
//impl Type<Postgres> for u64 {
//    fn type_info() -> PgTypeInfo {
//        PgTypeInfo::INT8
//    }
//}
//
//impl Encode<'_, Postgres> for u64 {
//	fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
//		buf.extend(&self.to_be_bytes());
//
//		Ok(IsNull::No)
//	}
//}
//
//impl Decode<'_, Postgres> for u64 {
//	fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
//		// Copied from source.
//		// https://docs.rs/sqlx-postgres/0.8.6/src/sqlx_postgres/types/int.rs.html#10
//		Ok(match value.format() {
//			PgValueFormat::Text => value.as_str()?.parse()?,
//			PgValueFormat::Binary => {
//				let buf = value.as_bytes()?;
//
//				// Return error if buf is empty or is more than 8 bytes
//				match buf.len() {
//					0 => {
//						return Err("Value Buffer found empty while decoding to integer type".into());
//					}
//					buf_len @ 9.. => {
//						return Err(format!(
//							"Value Buffer exceeds 8 bytes while decoding to integer type. Buffer size = {} bytes ", buf_len
//						)
//						.into());
//					}
//					_ => {}
//				}
//
//				BigEndian::read_int(buf, buf.len())
//			}
//		})
//	}
//}
//