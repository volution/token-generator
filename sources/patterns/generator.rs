

use crate::prelude::*;


use ::z_tokens_runtime::crates::num_bigint::{
		BigUint,
	};




define_error! (pub GeneratorError, result : GeneratorResult);




pub struct GenerateAccumulator {
	pub atoms : Vec<Rb<Atom>>,
	pub value : Option<BigUint>,
}




pub fn generate_token (_pattern : impl AsRef<TokenPattern>, _randomizer : &mut (impl Randomizer + ?Sized)) -> GeneratorResult<Token> {
	
	let mut _accumulator = GenerateAccumulator {
			atoms : Vec::with_capacity (ATOM_VEC_CAPACITY),
			value : None,
		};
	
	generate_token_push (_pattern, _randomizer, &mut _accumulator) ?;
	
	let _token = Token {
			atoms : RbList::from_vec_rb (_accumulator.atoms),
		};
	
	Ok (_token)
}


pub fn generate_token_push (_pattern : impl AsRef<TokenPattern>, _randomizer : &mut (impl Randomizer + ?Sized), _accumulator : &mut GenerateAccumulator) -> GeneratorResult {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		TokenPattern::Tagged (_pattern, _) =>
			generate_token_push (_pattern, _randomizer, _accumulator),
		
		TokenPattern::Atom (_pattern) =>
			generate_atom_push (_pattern, _randomizer, _accumulator),
		
		TokenPattern::Sequence (_patterns, _separator) => {
			let _count = _patterns.len ();
			for (_index, _pattern) in _patterns.iter () .enumerate () {
				let (_before, _after) = generate_separator (_separator, _index, _count) ?;
				if let Some (_separator) = _before {
					let _atom = Rb::new (Atom::Separator (_separator));
					_accumulator.atoms.push (_atom);
				}
				generate_token_push (_pattern, _randomizer, _accumulator) ?;
				if let Some (_separator) = _after {
					let _atom = Rb::new (Atom::Separator (_separator));
					_accumulator.atoms.push (_atom);
				}
			}
			Ok (())
		}
		
		TokenPattern::Permutation (_patterns, _separator) => {
			let _patterns = {
					// NOTE:  =>  <https://docs.rs/rand/latest/src/rand/seq/mod.rs.html#586>
					// FIXME:  Verify correctness!
					let mut _shuffle : Vec<_> = _patterns.iter () .collect ();
					for _index in (1 .. _shuffle.len ()) .rev () {
						let _other = _randomizer.choose (_index + 1) .else_wrap (0x04e15307) ?;
						_shuffle.swap (_index, _other);
					}
					_shuffle
				};
			let _count = _patterns.len ();
			for (_index, _pattern) in _patterns.into_iter () .enumerate () {
				let (_before, _after) = generate_separator (_separator, _index, _count) ?;
				if let Some (_separator) = _before {
					let _atom = Rb::new (Atom::Separator (_separator));
					_accumulator.atoms.push (_atom);
				}
				generate_token_push (_pattern, _randomizer, _accumulator) ?;
				if let Some (_separator) = _after {
					let _atom = Rb::new (Atom::Separator (_separator));
					_accumulator.atoms.push (_atom);
				}
			}
			Ok (())
		}
		
		TokenPattern::PermutationPartial (_patterns, _separator, _subset) => {
			let _patterns = {
					// NOTE:  =>  <https://docs.rs/rand/latest/src/rand/seq/mod.rs.html#586>
					// FIXME:  Verify correctness!
					let mut _shuffle : Vec<_> = _patterns.iter () .collect ();
					for _index in (1 .. _shuffle.len ()) .rev () {
						let _other = _randomizer.choose (_index + 1) .else_wrap (0x04e15307) ?;
						_shuffle.swap (_index, _other);
					}
					_shuffle.truncate (*_subset);
					_shuffle
				};
			let _count = _patterns.len ();
			for (_index, _pattern) in _patterns.into_iter () .enumerate () {
				let (_before, _after) = generate_separator (_separator, _index, _count) ?;
				if let Some (_separator) = _before {
					let _atom = Rb::new (Atom::Separator (_separator));
					_accumulator.atoms.push (_atom);
				}
				generate_token_push (_pattern, _randomizer, _accumulator) ?;
				if let Some (_separator) = _after {
					let _atom = Rb::new (Atom::Separator (_separator));
					_accumulator.atoms.push (_atom);
				}
			}
			Ok (())
		}
		
		TokenPattern::Repeat (_pattern, _separator, _count) => {
			let _count = *_count;
			for _index in 0 .. _count {
				let (_before, _after) = generate_separator (_separator, _index, _count) ?;
				if let Some (_separator) = _before {
					let _atom = Rb::new (Atom::Separator (_separator));
					_accumulator.atoms.push (_atom);
				}
				generate_token_push (_pattern, _randomizer, _accumulator) ?;
				if let Some (_separator) = _after {
					let _atom = Rb::new (Atom::Separator (_separator));
					_accumulator.atoms.push (_atom);
				}
			}
			Ok (())
		}
		
		TokenPattern::Choice (_patterns) => {
			let _count = _patterns.len ();
			let _index = _randomizer.choose (_count) .else_wrap (0x57deb750) ?;
			let _pattern = _patterns[_index] .as_ref ();
			generate_token_push (_pattern, _randomizer, _accumulator)
		}
		
		TokenPattern::Empty =>
			Ok (()),
	}
}




pub fn generate_separator (_pattern : impl AsRef<SeparatorPattern>, _index : usize, _count : usize) -> GeneratorResult<(Option<Rb<Separator>>, Option<Rb<Separator>>)> {
	
	debug_assert! (_count > 0, "[5e054c68]");
	debug_assert! (_index < _count, "[ff47b883]");
	
	let _pattern = _pattern.as_ref ();
	let _separators = match _pattern {
		
		SeparatorPattern::None =>
			(None, None),
		
		SeparatorPattern::Prefix (_prefix) =>
			(
				if _index == 0 { Some (_prefix.clone ()) } else { None },
				None,
			),
		
		SeparatorPattern::Suffix (_suffix) =>
			(
				None,
				if _index == (_count - 1) { Some (_suffix.clone ()) } else { None },
			),
		
		SeparatorPattern::Bracket (_prefix, _suffix) =>
			(
				if _index == 0 { Some (_prefix.clone ()) } else { None },
				if _index == (_count - 1) { Some (_suffix.clone ()) } else { None },
			),
		
		SeparatorPattern::Infix (_separator) =>
			(
				if _index > 0 { Some (_separator.clone ()) } else { None },
				None,
			),
		
		SeparatorPattern::InfixEach (_separator, _each) =>
			(
				if (_index > 0) && ((_index % _each) == 0) { Some (_separator.clone ()) } else { None },
				None,
			),
	};
	Ok (_separators)
}




pub fn generate_atom_push (_pattern : impl AsRef<AtomPattern>, _randomizer : &mut (impl Randomizer + ?Sized), _accumulator : &mut GenerateAccumulator) -> GeneratorResult {
	let _pattern = _pattern.as_ref ();
	match _pattern {
		
		AtomPattern::Separator (_separator) => {
			let _separator = _separator.clone ();
			let _atom = Rb::new (Atom::Separator (_separator));
			_accumulator.atoms.push (_atom);
			Ok (())
		}
		
		AtomPattern::Constant (_text) => {
			let _text = _text.clone ();
			let _atom = Rb::new (Atom::Constant (_text));
			_accumulator.atoms.push (_atom);
			Ok (())
		}
		
		AtomPattern::Glyph (_pattern) =>
			generate_glyph_push (_pattern, _randomizer, _accumulator),
	}
}




pub fn generate_glyph_push (_pattern : impl AsRef<GlyphPattern>, _randomizer : &mut (impl Randomizer + ?Sized), _accumulator : &mut GenerateAccumulator) -> GeneratorResult {
	let _pattern = _pattern.as_ref ();
	let (_glyph, _count, _index) = match _pattern {
		
		GlyphPattern::Set (_patterns) => {
			let _count = _patterns.len ();
			let _index = _randomizer.choose (_count) .else_wrap (0x80578f69) ?;
			let _glyph = _patterns[_index] .clone ();
			(_glyph, _count, _index)
		}
		
		GlyphPattern::Integer (_lower, _upper, _format) => {
			let (_lower, _upper) = (*_lower, *_upper);
			if _lower > _upper {
				fail! (0xb8a08c0e);
			}
			let _delta = _upper - _lower;
			let (_value, _count, _index) = if _delta == 0 {
				(_lower, 1, 0)
			} else if _delta == u128::MAX {
				let _random = _randomizer.value_u128 () .else_wrap (0x33ed6d05) ?;
				//  FIXME!
				(_random, 0, 0)
			} else if (_delta + 1) <= (usize::MAX as u128) {
				let _count = (_delta + 1) as usize;
				let _index = _randomizer.choose (_count) .else_wrap (0x5079d3d3) ?;
				let _value = _lower + (_index as u128);
				(_value, _count, _index)
			} else if (_delta.leading_zeros () + _delta.trailing_ones ()) == 128 {
				let _random = _randomizer.value_u128 () .else_wrap (0xb82829eb) ?;
				let _index = _random & _delta;
				let _value = _lower + _index;
				//  FIXME!
				(_value, 0, 0)
			} else {
				::std::eprintln! ("{} {} / {} {:x} {:b}", _delta.leading_zeros (), _delta.trailing_ones (), _delta, _delta, _delta);
				fail! (0x4cc61b73);
			};
			let _glyph = Glyph::Integer (Rb::new (_value), *_format);
			let _glyph = Rb::new (_glyph);
			(_glyph, _count, _index)
		}
		
		GlyphPattern::Bytes (_size, _format) => {
			let _size = *_size;
			let mut _bytes = vec! [0; _size];
			_randomizer.bytes (&mut _bytes) .else_wrap (0x6f1ec700) ?;
			if let Some (_accumulator_value) = &mut _accumulator.value {
				for _byte in _bytes.iter () {
					*_accumulator_value <<= 8;
					*_accumulator_value += *_byte;
				}
			}
			let _bytes = Bytes::Boxed (_bytes.into_boxed_slice ());
			let _glyph = Glyph::Bytes (Rb::new (_bytes), *_format);
			let _glyph = Rb::new (_glyph);
			(_glyph, 0, 0)
		}
		
		GlyphPattern::Timestamp (_format) => {
			let _timestamp = _randomizer.timestamp () .else_wrap (0x94acb952) ?;
			let _glyph = Glyph::Timestamp (Rb::new (_timestamp), *_format);
			let _glyph = Rb::new (_glyph);
			(_glyph, 0, 0)
		}
	};
	
	let _atom = Rb::new (Atom::Glyph (_glyph));
	_accumulator.atoms.push (_atom);
	
	if let Some (_accumulator_value) = &mut _accumulator.value {
		if _count > 1 {
			*_accumulator_value *= _count;
		}
		if _index > 0 {
			*_accumulator_value += _index;
		}
	}
	
	Ok (())
}

