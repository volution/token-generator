

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;

use crate::model::*;
use crate::inputs::Input;

use ::digest::{
		self as digest,
		Digest as _,
		VariableOutput as _,
	};




define_error! (pub HashError, result : HashResult);




pub fn hash (_algorithm : Algorithm, _output_size : usize, _input : impl Input) -> HashResult<Vec<u8>> {
	
	if _output_size == 0 {
		fail! (0x93d0f3af);
	}
	if _output_size > OUTPUT_SIZE_MAX {
		fail! (0x32c196e2);
	}
	
	let mut _output = vec! [0u8; _output_size];
	
	match _algorithm {
		
		Algorithm::MD5 =>
			hash_fixed (::md5::Md5::new (), _input, &mut _output) ?,
		
		Algorithm::SHA1 =>
			hash_fixed (::sha1::Sha1::new (), _input, &mut _output) ?,
		
		Algorithm::SHA2_224 =>
			hash_fixed (::sha2::Sha224::new (), _input, &mut _output) ?,
		Algorithm::SHA2_256 =>
			hash_fixed (::sha2::Sha256::new (), _input, &mut _output) ?,
		Algorithm::SHA2_384 =>
			hash_fixed (::sha2::Sha384::new (), _input, &mut _output) ?,
		Algorithm::SHA2_512 =>
			hash_fixed (::sha2::Sha512::new (), _input, &mut _output) ?,
		
		Algorithm::SHA3_224 =>
			hash_fixed (::sha3::Sha3_224::new (), _input, &mut _output) ?,
		Algorithm::SHA3_256 =>
			hash_fixed (::sha3::Sha3_256::new (), _input, &mut _output) ?,
		Algorithm::SHA3_384 =>
			hash_fixed (::sha3::Sha3_384::new (), _input, &mut _output) ?,
		Algorithm::SHA3_512 =>
			hash_fixed (::sha3::Sha3_512::new (), _input, &mut _output) ?,
		
		Algorithm::GitSHA1 =>
			fail! (0x64e83dae),
		
		Algorithm::Blake2s =>
			hash_variable (::blake2::Blake2sVar::new (_output_size) .else_wrap (0xfb4c3bb9) ?, _input, &mut _output) ?,
		Algorithm::Blake2b =>
			hash_variable (::blake2::Blake2bVar::new (_output_size) .else_wrap (0x6e7b8e58) ?, _input, &mut _output) ?,
		
		Algorithm::Blake3 =>
			hash_extendable (::blake3::Hasher::new (), _input, &mut _output) ?,
		
		Algorithm::Argon2d =>
			hash_argon (::argon2::Algorithm::Argon2d, _input, &mut _output) ?,
		Algorithm::Argon2i =>
			hash_argon (::argon2::Algorithm::Argon2i, _input, &mut _output) ?,
		Algorithm::Argon2id =>
			hash_argon (::argon2::Algorithm::Argon2id, _input, &mut _output) ?,
		
		Algorithm::XxHash_32 =>
			hash_xxhash_32 (_input, &mut _output) ?,
		Algorithm::XxHash_64 =>
			hash_xxhash_64 (_input, &mut _output) ?,
		
		Algorithm::Xxh3_64 =>
			hash_xxh3_64 (_input, &mut _output) ?,
		Algorithm::Xxh3_128 =>
			hash_xxh3_128 (_input, &mut _output) ?,
		
	}
	
	Ok (_output)
}








fn hash_fixed <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8]) -> HashResult
		where Hasher : digest::FixedOutput + digest::Update
{
	hash_update (&mut _hasher, _input) .else_wrap (0x3322631d) ?;
	
	let _output_size = _output.len ();
	let _hash_full = _hasher.finalize_fixed ();
	if _hash_full.len () < _output_size {
		fail! (0x529b2c3f);
	}
	
	_output.copy_from_slice (&_hash_full[0.._output_size]);
	
	Ok (())
}


fn hash_variable <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8]) -> HashResult
		where Hasher : digest::VariableOutput + digest::Update
{
	hash_update (&mut _hasher, _input) .else_wrap (0xccfa4243) ?;
	
	_hasher.finalize_variable (_output) .else_wrap (0x52d8d078) ?;
	
	Ok (())
}


fn hash_extendable <Hasher> (mut _hasher : Hasher, _input : impl Input, _output : &mut [u8]) -> HashResult
		where Hasher : digest::ExtendableOutput + digest::Update
{
	hash_update (&mut _hasher, _input) .else_wrap (0x5df214fb) ?;
	
	_hasher.finalize_xof_into (_output);
	
	Ok (())
}


fn hash_update <Hasher> (_hasher : &mut Hasher, mut _input : impl Input) -> HashResult
		where Hasher : digest::Update
{
	while let Some (_data) = _input.input () .else_wrap (0x17507faa) ? {
		_hasher.update (_data);
	}
	
	Ok (())
}








fn hash_xxhash_32 (_input : impl Input, _output : &mut [u8]) -> HashResult {
	
	let mut _hasher = ::twox_hash::XxHash32::with_seed (0);
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher) as u32;
	
	let mut _hash_bytes = [0u8; 4];
	use ::byteorder::ByteOrder as _;
	::byteorder::BigEndian::write_u32 (&mut _hash_bytes, _hash_value);
	
	copy_output_from_slice (&_hash_bytes, _output)
}


fn hash_xxhash_64 (_input : impl Input, _output : &mut [u8]) -> HashResult {
	
	let mut _hasher = ::twox_hash::XxHash64::with_seed (0);
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher);
	
	let mut _hash_bytes = [0u8; 8];
	use ::byteorder::ByteOrder as _;
	::byteorder::BigEndian::write_u64 (&mut _hash_bytes, _hash_value);
	
	copy_output_from_slice (&_hash_bytes, _output)
}




fn hash_xxh3_64 (_input : impl Input, _output : &mut [u8]) -> HashResult {
	
	let mut _hasher = ::twox_hash::Xxh3Hash64::with_seed (0);
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = Hasher::finish (&_hasher);
	
	let mut _hash_bytes = [0u8; 8];
	use ::byteorder::ByteOrder as _;
	::byteorder::BigEndian::write_u64 (&mut _hash_bytes, _hash_value);
	
	copy_output_from_slice (&_hash_bytes, _output)
}


fn hash_xxh3_128 (_input : impl Input, _output : &mut [u8]) -> HashResult {
	
	let mut _hasher = ::twox_hash::Xxh3Hash128::with_seed (0);
	hash_update_std (&mut _hasher, _input) ?;
	let _hash_value = ::twox_hash::xxh3::HasherExt::finish_ext (&_hasher);
	
	let mut _hash_bytes = [0u8; 16];
	use ::byteorder::ByteOrder as _;
	::byteorder::BigEndian::write_u128 (&mut _hash_bytes, _hash_value);
	
	copy_output_from_slice (&_hash_bytes, _output)
}




fn hash_update_std <Hasher> (_hasher : &mut Hasher, mut _input : impl Input) -> HashResult
		where Hasher : hash::Hasher
{
	while let Some (_data) = _input.input () .else_wrap (0x397076d5) ? {
		_hasher.write (_data);
	}
	
	Ok (())
}




fn copy_output_from_slice (_hash : &[u8], _output : &mut [u8]) -> HashResult {
	
	let _output_len = _output.len ();
	
	if _hash.len () < _output_len {
		fail! (0xedf869a5);
	}
	
	_output.copy_from_slice (&_hash[.. _output_len]);
	
	Ok (())
}








fn hash_argon (_algorithm : ::argon2::Algorithm, mut _input : impl Input, _output : &mut [u8]) -> HashResult {
	
	const M_COST_MAX : u32 = 1024 * 1024;
	const M_COST_BASE : u32 = 16 * 1024;
	const T_COST_BASE : u32 = 8;
	const P_COST : u32 = 1;
	
	let _output_size = _output.len ();
	let _m_cost = u32::min (_output_size as u32 * M_COST_BASE, M_COST_MAX);
	let _t_cost = u32::max (_output_size as u32 * T_COST_BASE / (M_COST_MAX / M_COST_BASE / 4), T_COST_BASE);
	
//	::std::eprintln! ("[dd] [65403625]  output {}, m_cost {}, t_cost {}", _output_size, _m_cost / 1024, _t_cost);
	
	let mut _input_hash = vec! [0u8; 64];
	hash_fixed (::blake2::Blake2b512::new (), _input, &mut _input_hash) ?;
	
	let _hasher_parameters = ::argon2::Params::new (_m_cost, _t_cost, P_COST, Some (_output_size)) .else_wrap (0x8acd25cd) ?;
	let _hasher = ::argon2::Argon2::new (_algorithm, ::argon2::Version::V0x13, _hasher_parameters);
	
	_hasher.hash_password_into (&_input_hash, &_input_hash, _output) .else_wrap (0xce42692d) ?;
	
	Ok (())
}


