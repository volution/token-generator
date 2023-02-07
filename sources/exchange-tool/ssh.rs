

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;


use ::russh_keys::{
		key::PublicKey as SshPublicKey,
		signature::Signature as SshSignature,
		agent::client::AgentClient as SshAgentClient,
		PublicKeyBase64 as _,
	};


use ::tokio::{
		runtime::{
			Runtime,
			Builder as RuntimeBuilder,
		},
		net::UnixStream,
	};


use ::z_tokens_runtime::{
		memory::Rb,
		memory::RbList,
		sensitive::Sensitive,
		sensitive::SensitiveIgnored,
		sensitive::zeroize_and_drop,
	};

use crate::keys::{
		encode_raw,
		decode_raw_vec,
	};

use crate::low::*;
use crate::macros::*;








define_error! (pub SshError, result : SshResult);




pub const SSH_WRAPPER_KEY_ENCODED_PREFIX : &str = "ztxws";


define_cryptographic_material! (InternalSshWrapInput, input, slice);
define_cryptographic_material! (InternalSshWrapKeyHash, 32);
define_cryptographic_material! (InternalSshWrapInputHash, 32);
define_cryptographic_material! (InternalSshWrapOutputHash, 32);


define_cryptographic_context! (SSH_WRAP_KEY_HASH_CONTEXT, ssh_wrap, key_hash);
define_cryptographic_context! (SSH_WRAP_INPUT_HASH_CONTEXT, ssh_wrap, input_hash);
define_cryptographic_context! (SSH_WRAP_OUTPUT_HASH_CONTEXT, ssh_wrap, output_hash);








pub struct SshWrapper {
	key : Rb<SshWrapperKey>,
	agent : SshWrapperAgent,
}


pub struct SshWrapperKey {
	public_key : Rb<SensitiveIgnored<SshPublicKey>>,
}


pub struct SshWrapperAgent {
	runtime : Runtime,
	client : Option<SshAgentClient<UnixStream>>,
}








impl SshWrapper {
	
	
	pub fn new (_key : impl Into<Rb<SshWrapperKey>>, _agent : SshWrapperAgent) -> SshResult<SshWrapper> {
		let _key = _key.into ();
		let _self = SshWrapper {
				key : _key,
				agent : _agent,
			};
		Ok (_self)
	}
	
	
	pub fn into_agent (self) -> SshResult<SshWrapperAgent> {
		Ok (self.agent)
	}
	
	
	pub fn connect (_key : impl Into<Rb<SshWrapperKey>>) -> SshResult<SshWrapper> {
		
		let _agent = SshWrapperAgent::connect () ?;
		Self::new (_key, _agent)
	}
	
	
	pub fn wrap (&mut self, _input : &[u8], _output : &mut [u8; 32]) -> SshResult {
		
		let _input = InternalSshWrapInput::wrap (_input);
		
		let _key = &self.key.public_key.0;
		let _key_algorithm = _key.name ();
		let _key_serialized = _key.public_key_bytes ();
		
		let _key_hash = blake3_derive_key (
				InternalSshWrapKeyHash::wrap,
				SSH_WRAP_KEY_HASH_CONTEXT,
				&[],
				&[
					_key_algorithm.as_bytes (),
					&_key_serialized,
				],
				None,
			);
		
		drop! (_key_algorithm, _key_serialized);
		
		let _input_hash = blake3_derive_key (
				InternalSshWrapInputHash::wrap,
				SSH_WRAP_INPUT_HASH_CONTEXT,
				&[
					_key_hash.access (),
				],
				&[
					_input.access (),
				],
				None,
			);
		
		let _outcome = {
			
			let _runtime = &mut self.agent.runtime;
			let mut _client = self.agent.client.take () .else_wrap (0xa5bc5a47) ?;
			
			let (_client, _outcome) = self.agent.runtime.block_on (async {
					_client.sign_request_signature (_key, _input_hash.access ()) .await
				});
			
			self.agent.client = Some (_client);
			
			_outcome
		};
		
		let _signature = _outcome.else_wrap (0xe3badadd) ?;
		let _signature : &[u8] = match _signature {
			SshSignature::Ed25519 (ref _bytes) => &_bytes.0,
			SshSignature::RSA { hash : _, bytes : ref _bytes } => &_bytes,
		};
		
		if ! _key.verify_detached (_input_hash.access (), &_signature) {
			fail! (0x8fc8e73a);
		}
		
		drop! (_key);
		
		let _output_hash = blake3_derive_key (
				InternalSshWrapOutputHash::wrap,
				SSH_WRAP_OUTPUT_HASH_CONTEXT,
				&[
					_key_hash.access (),
					_input_hash.access (),
				],
				&[
					&_signature,
				],
				None,
			);
		
		_output.copy_from_slice (_output_hash.access ());
		
		Ok (())
	}
	
	
	pub fn cmp_by_keys (_left : &Self, _right : &Self) -> Ordering {
		
		let _left_key = &_left.key.public_key.0;
		let _right_key = &_right.key.public_key.0;
		
		let _left_key_algorithm = _left_key.name ();
		let _right_key_algorithm = _right_key.name ();
		
		Ord::cmp (_left_key_algorithm, _right_key_algorithm) .then_with (
				|| {
					let _left_key_serialized = _left_key.public_key_bytes ();
					let _right_key_serialized = _right_key.public_key_bytes ();
					Ord::cmp (&_left_key_serialized, &_right_key_serialized)
				})
	}
}








impl SshWrapperKey {
	
	
	pub fn encode (&self) -> SshResult<Rb<String>> {
		
		let _key : &SshPublicKey = &self.public_key.0;
		
		let _algorithm = _key.name ();
		let mut _serialized = _key.public_key_bytes ();
		
		let _code = match _algorithm {
			"ssh-ed25519" => 1,
			"ssh-rsa" => 2,
			_rsa if _rsa.starts_with ("rsa-") => 2,
			_ => fail! (0x3cb34ad5),
		};
		
		_serialized.push (_code);
		
		encode_raw (SSH_WRAPPER_KEY_ENCODED_PREFIX, &_serialized) .else_wrap (0xc28cbe1d)
	}
	
	
	pub fn decode_and_zeroize (_string : String) -> SshResult<Self> {
		let _outcome = Self::decode (&_string);
		zeroize_and_drop (_string);
		_outcome
	}
	
	pub fn decode (_string : &str) -> SshResult<Self> {
		
		let mut _serialized = decode_raw_vec (SSH_WRAPPER_KEY_ENCODED_PREFIX, _string) .else_wrap (0x142d169b) ?;
		let _code = _serialized.pop () .else_wrap (0x428e8a31) ?;
		
		let _algorithm = match _code {
			1 => "ssh-ed25519",
			2 => "ssh-rsa",
			_ => fail! (0xaec55807),
		};
		
		let _key = SshPublicKey::parse (_algorithm.as_bytes (), &_serialized) .else_wrap (0x536edcf6) ?;
		
		let _wrapper_key = SshWrapperKey {
				public_key : Rb::new (_key.clone () .into ()),
			};
		
		Ok (_wrapper_key)
	}
	
	
	pub fn handle (&self) -> SshResult<Rb<String>> {
		let _key : &SshPublicKey = &self.public_key.0;
		let _algorithm = _key.name ();
		let _fingerprint = _key.fingerprint ();
		let _handle = format! ("{}:{}", _algorithm, _fingerprint);
		zeroize_and_drop (_fingerprint);
		Ok (Rb::new (_handle))
	}
}


impl Sensitive for SshWrapperKey {
	
	fn erase (&mut self) -> () {
		self.public_key.erase ();
	}
}








impl SshWrapperAgent {
	
	
	pub fn connect () -> SshResult<SshWrapperAgent> {
		
		let mut _runtime =
				RuntimeBuilder::new_current_thread ()
				.enable_io ()
				.build ()
				.else_wrap (0x2159c9b5) ?;
		
		let mut _client = _runtime.block_on (async {
				SshAgentClient::connect_env () .await
			}) .else_wrap (0x67dc4fca) ?;
		
		let _wrapper = SshWrapperAgent {
				runtime : _runtime,
				client : Some (_client),
			};
		
		Ok (_wrapper)
	}
	
	
	pub fn keys (&mut self) -> SshResult<RbList<SshWrapperKey>> {
		
		let mut _client = self.client.take () .else_wrap (0xbe88e692) ?;
		
		let _keys = self.runtime.block_on (async {
				_client.request_identities () .await
			}) .else_wrap (0xbad56d04) ?;
		
		self.client = Some (_client);
		
		let mut _wrapper_keys = Vec::with_capacity (_keys.len ());
		
		for _key in _keys {
			let _algorithm = _key.name ();
			if ! ((_algorithm == "ssh-ed25519") || (_algorithm == "ssh-rsa") || _algorithm.starts_with ("rsa-")) {
				continue;
			}
			
			let _wrapper_key = SshWrapperKey {
					public_key : Rb::new (_key.clone () .into ()),
				};
			
			_wrapper_keys.push (_wrapper_key);
		}
		
		let _wrapper_keys = RbList::from_vec (_wrapper_keys);
		
		Ok (_wrapper_keys)
	}
}


