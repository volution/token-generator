

#![ allow (unused_macros) ]
#![ allow (unused_imports) ]




use crate::prelude::*;


use crate::glyphs as glyphs;
use crate::separators as separators;
use crate::macros as macros;


include! ("./tokens_macros.in");








#[ cfg (feature = "zt-patterns-digits-base10") ]
define_repeat! (
		pub DIGITS_BASE10,
		("digits-base10", "encoding", "digits", "password", "pronounceable", "memorable"),
		("digits-base10", "d", "pin"),
		{ glyphs::DIGIT_BASE10_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base2") ]
define_repeat! (
		pub DIGITS_BASE2,
		("digits-base2", "encoding", "digits"),
		("digits-base2"),
		{ glyphs::DIGIT_BASE2_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN },
		(32 : 4));

#[ cfg (feature = "zt-patterns-digits-base8") ]
define_repeat! (
		pub DIGITS_BASE8,
		("digits-base8", "encoding", "digits"),
		("digits-base8"),
		{ glyphs::DIGIT_BASE8_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base16") ]
define_repeat! (
		pub DIGITS_BASE16,
		("digits-base16", "encoding", "password"),
		("digits-base16", "x"),
		{ glyphs::DIGIT_BASE16_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));


#[ cfg (feature = "zt-patterns-digits-base32") ]
define_repeat! (
		pub DIGITS_BASE32_HEX,
		("digits-base32-hex", "digits-base32", "encoding", "password"),
		("digits-base32-hex"),
		{ glyphs::DIGIT_BASE32_HEX_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base32") ]
define_repeat! (
		pub DIGITS_BASE32_RFC,
		("digits-base32-rfc", "digits-base32", "encoding", "password"),
		("digits-base32-rfc"),
		{ glyphs::DIGIT_BASE32_RFC_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN},
		(32 : 2));


#[ cfg (feature = "zt-patterns-digits-base64") ]
define_repeat! (
		pub DIGITS_BASE64_URL,
		("digits-base64-url", "digits-base64", "encoding", "password"),
		("digits-base64-url"),
		{ glyphs::DIGIT_BASE64_URL_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN},
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base64") ]
define_repeat! (
		pub DIGITS_BASE64_RFC,
		("digits-base64-rfc", "digits-base64", "encoding", "password"),
		("digits-base64-rfc"),
		{ glyphs::DIGIT_BASE64_RFC_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN},
		(32 : 2));


#[ cfg (feature = "zt-patterns-digits-base58") ]
define_repeat! (
		pub DIGITS_BASE58,
		("digits-base58", "encoding", "password"),
		("digits-base58"),
		{ glyphs::DIGIT_BASE58_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-base62") ]
define_repeat! (
		pub DIGITS_BASE62,
		("digits-base62", "encoding", "password"),
		("digits-base62"),
		{ glyphs::DIGIT_BASE62_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-bech32") ]
define_repeat! (
		pub DIGITS_BECH32,
		("digits-bech32", "encoding", "password"),
		("digits-bech32"),
		{ glyphs::DIGIT_BECH32_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-digits-z85") ]
define_repeat! (
		pub DIGITS_Z85,
		("digits-z85", "encoding", "password"),
		("digits-z85"),
		{ glyphs::DIGIT_Z85_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_5_PATTERN },
		(35 : 5));








#[ cfg (feature = "zt-patterns-bytes") ]
define_bytes! (
		pub BYTES_HEX,
		("bytes-hex", "encoding", "password"),
		("bytes-hex", "b"),
		BYTES_HEX,
		(512 : 1));








#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_LETTER_LOWER,
		("ascii-lower", "letters", "password"),
		("ascii-lower"),
		{ glyphs::ASCII_LETTER_LOWER_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_LETTER_UPPER,
		("ascii-upper", "letters"),
		("ascii-upper"),
		{ glyphs::ASCII_LETTER_UPPER_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_LETTER_MIXED,
		("ascii-mixed", "letters", "password"),
		("ascii-mixed"),
		{ glyphs::ASCII_LETTER_MIXED_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));


#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_SYMBOLS,
		("ascii-symbols"),
		("ascii-symbols"),
		{ glyphs::ASCII_SYMBOL_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));

#[ cfg (feature = "zt-patterns-ascii") ]
define_repeat! (
		pub ASCII_PRINTABLE,
		("ascii-any", "password"),
		("ascii-any", "r"),
		{ glyphs::ASCII_PRINTABLE_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(32 : 2));








#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_LOWER_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_LOWER_TOKEN,
			glyphs::ASCII_VOWEL_LOWER_TOKEN,
			glyphs::ASCII_CONSONANT_LOWER_TOKEN,
			glyphs::ASCII_VOWEL_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_UPPER_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_MIXED_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_MIXED_TOKEN,
			glyphs::ASCII_VOWEL_MIXED_TOKEN,
			glyphs::ASCII_CONSONANT_MIXED_TOKEN,
			glyphs::ASCII_VOWEL_MIXED_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_LOWER,
		("cva-lower", "cva", "cv", "letters", "password", "pronounceable", "memorable"),
		("cva-lower", "cva"),
		{ ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_UPPER,
		("cva-upper", "cva", "cv", "letters"),
		("cva-upper"),
		{ ASCII_CONSONANT_VOWEL_UPPER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_MIXED,
		("cva-mixed", "cva", "cv", "letters", "password"),
		("cva-mixed"),
		{ ASCII_CONSONANT_VOWEL_MIXED_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));




#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_PLUS_A_WORD,
		(),
		(),
		[
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_PLUS_B_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub ASCII_CONSONANT_VOWEL_PLUS_C_WORD,
		(),
		(),
		[
			glyphs::ASCII_CONSONANT_UPPER_TOKEN,
			glyphs::ASCII_VOWEL_UPPER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::ASCII_SYMBOL_TOKEN,
		], separators::NONE_PATTERN);


#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_PLUS_A,
		("cva-plus-a", "cva-plus", "cv-plus", "cva", "cv", "password", "pronounceable", "memorable"),
		("cva-plus-a", "cvapa"),
		{
			(),
			( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_A_WORD, )
		},
		(15 : 1, number_plus_one));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_PLUS_B,
		("cva-plus-b", "cva-plus", "cv-plus", "cva", "cv", "password", "pronounceable", "memorable"),
		("cva-plus-b", "cvapb"),
		{
			(),
			( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_B_WORD, )
		},
		(15 : 1, number_plus_one));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub ASCII_CONSONANT_VOWEL_PLUS_C,
		("cva-plus-c", "cva-plus", "cv-plus", "cva", "cv", "password", "pronounceable", "memorable"),
		("cva-plus-c", "cvapc"),
		{
			(),
			( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_C_WORD, )
		},
		(15 : 1, number_plus_one));








#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_LOWER_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_LOWER_TOKEN,
			glyphs::SIMPLE_VOWEL_LOWER_TOKEN,
			glyphs::SIMPLE_CONSONANT_LOWER_TOKEN,
			glyphs::SIMPLE_VOWEL_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_UPPER_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_UPPER_TOKEN,
			glyphs::SIMPLE_VOWEL_UPPER_TOKEN,
			glyphs::SIMPLE_CONSONANT_UPPER_TOKEN,
			glyphs::SIMPLE_VOWEL_UPPER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_MIXED_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_MIXED_TOKEN,
			glyphs::SIMPLE_VOWEL_MIXED_TOKEN,
			glyphs::SIMPLE_CONSONANT_MIXED_TOKEN,
			glyphs::SIMPLE_VOWEL_MIXED_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_LOWER,
		("cvs-lower", "cvs", "cv", "letters", "password", "pronounceable", "memorable"),
		("cvs-lower", "cvs"),
		{ SIMPLE_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_UPPER,
		("cvs-upper", "cvs", "cv", "letters"),
		("cvs-upper"),
		{ SIMPLE_CONSONANT_VOWEL_UPPER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_MIXED,
		("cvs-mixed", "cvs", "cv", "letters", "password"),
		("cvs-mixed"),
		{ SIMPLE_CONSONANT_VOWEL_MIXED_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));




#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_A_WORD,
		(),
		(),
		[
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_B_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_UPPER_TOKEN,
			glyphs::SIMPLE_VOWEL_UPPER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_sequence! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_C_WORD,
		(),
		(),
		[
			glyphs::SIMPLE_CONSONANT_UPPER_TOKEN,
			glyphs::SIMPLE_VOWEL_UPPER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::ASCII_SYMBOL_TOKEN,
		], separators::NONE_PATTERN);


#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_A,
		("cvs-plus-a", "cvs-plus", "cv-plus", "cvs", "cv", "password", "pronounceable", "memorable"),
		("cvs-plus-a", "cvspa"),
		{
			(),
			( SIMPLE_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, SIMPLE_CONSONANT_VOWEL_PLUS_A_WORD, )
		},
		(15 : 1, number_plus_one));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_B,
		("cvs-plus-b", "cvs-plus", "cv-plus", "cvs", "cv", "password", "pronounceable", "memorable"),
		("cvs-plus-b", "cvspb"),
		{
			(),
			( SIMPLE_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, SIMPLE_CONSONANT_VOWEL_PLUS_B_WORD, )
		},
		(15 : 1, number_plus_one));

#[ cfg (feature = "zt-patterns-consonant-vowel") ]
define_repeat! (
		pub SIMPLE_CONSONANT_VOWEL_PLUS_C,
		("cvs-plus-c", "cvs-plus", "cv-plus", "cvs", "cv", "password", "pronounceable", "memorable"),
		("cvs-plus-c", "cvspc"),
		{
			(),
			( SIMPLE_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
			( separators::SPACE_OPTIONAL_TOKEN, SIMPLE_CONSONANT_VOWEL_PLUS_C_WORD, )
		},
		(15 : 1, number_plus_one));








// NOTE:  =>  <https://rmondello.com/2024/10/07/apple-passwords-generated-strong-password-format/>


#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_1,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
		], separators::NONE_PATTERN);


#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_2A,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_UPPER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_2B,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_UPPER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_2C,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_UPPER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_2D,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_UPPER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_2E,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_UPPER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_2F,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_UPPER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_choice! (
		pub APPLE_WORD_2,
		(),
		(),
		[
			APPLE_WORD_2A,
			APPLE_WORD_2B,
			APPLE_WORD_2C,
			APPLE_WORD_2D,
			APPLE_WORD_2E,
			APPLE_WORD_2F,
		]);


#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_3A,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_3B,
		(),
		(),
		[
			glyphs::DIGIT_BASE10_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_choice! (
		pub APPLE_WORD_3,
		(),
		(),
		[
			APPLE_WORD_3A,
			APPLE_WORD_3B,
		]);


#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_WORD_4,
		(),
		(),
		[
			glyphs::APPLE_CONSONANT_UPPER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_CONSONANT_LOWER_TOKEN,
			glyphs::APPLE_VOWEL_LOWER_TOKEN,
			glyphs::DIGIT_BASE10_TOKEN,
		], separators::NONE_PATTERN);


#[ cfg (feature = "zt-patterns-apple") ]
define_permutation! (
		pub APPLE_ORIGINAL,
		("apple-original", "apple", "cva-plus", "cv-plus", "cva", "cv", "password", "pronounceable", "memorable"),
		("apple-original", "apple"),
		[
			APPLE_WORD_1,
			APPLE_WORD_2,
			APPLE_WORD_3,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_LOWER,
		("apple-lower", "apple-variant", "apple", "cva", "cv", "password", "pronounceable", "memorable"),
		("apple-lower", "apple-l"),
		[
			APPLE_WORD_1,
			APPLE_WORD_1,
			APPLE_WORD_1,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_sequence! (
		pub APPLE_VARIANT_A,
		("apple-variant-a", "apple-variant", "apple", "cva-plus", "cv-plus", "cva", "cv", "password", "pronounceable", "memorable"),
		("apple-variant-a", "apple-a"),
		[
			APPLE_WORD_4,
			APPLE_WORD_1,
			APPLE_WORD_1,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-apple") ]
define_all! (pub APPLE_ALL, [
		APPLE_ORIGINAL,
		APPLE_LOWER,
		APPLE_VARIANT_A,
	]);








#[ cfg (feature = "zt-patterns-proquint") ]
define_sequence! (
		pub PROQUINT_LOWER_WORD,
		(),
		(),
		[
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
			glyphs::PROQUINT_VOWEL_LOWER_TOKEN,
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
			glyphs::PROQUINT_VOWEL_LOWER_TOKEN,
			glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-proquint") ]
define_sequence! (
		pub PROQUINT_UPPER_WORD,
		(),
		(),
		[
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
			glyphs::PROQUINT_VOWEL_UPPER_TOKEN,
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
			glyphs::PROQUINT_VOWEL_UPPER_TOKEN,
			glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-proquint") ]
define_repeat! (
		pub PROQUINT_LOWER,
		("proquint-lower", "proquint", "letters", "password", "pronounceable", "memorable", "encoding"),
		("proquint-lower", "proquint"),
		{ PROQUINT_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-proquint") ]
define_repeat! (
		pub PROQUINT_UPPER,
		("proquint-upper", "proquint", "letters"),
		("proquint-upper"),
		{ PROQUINT_UPPER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));








#[ cfg (feature = "zt-patterns-koremutake") ]
define_sequence! (
		pub KOREMUTAKE_WORD_A,
		(),
		(),
		[
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-koremutake") ]
define_sequence! (
		pub KOREMUTAKE_WORD_B,
		(),
		(),
		[
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
			glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
		], separators::NONE_PATTERN);

#[ cfg (feature = "zt-patterns-koremutake") ]
define_repeat! (
		pub KOREMUTAKE_A,
		("koremutake-a", "koremutake", "letters", "password", "pronounceable", "memorable", "encoding"),
		("koremutake-a"),
		{ KOREMUTAKE_WORD_A => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));

#[ cfg (feature = "zt-patterns-koremutake") ]
define_repeat! (
		pub KOREMUTAKE_B,
		("koremutake-b", "koremutake", "letters", "password", "pronounceable", "memorable", "encoding"),
		("koremutake-b"),
		{ KOREMUTAKE_WORD_B => separators::SPACE_OPTIONAL_INFIX_PATTERN },
		(16 : 1));








#[ cfg (feature = "zt-patterns-mnemonic") ]
define_repeat! (
		pub MNEMONIC,
		("mnemonic", "dictionary", "passphrase", "pronounceable", "memorable", "encoding"),
		("mnemonic"),
		{ glyphs::MNEMONIC_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));




#[ cfg (feature = "zt-patterns-bip0039") ]
define_repeat! (
		pub BIP0039,
		("bip0039", "dictionary", "passphrase", "pronounceable", "memorable", "encoding"),
		("bip0039"),
		{ glyphs::BIP0039_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));




#[ cfg (feature = "zt-patterns-skey") ]
define_repeat! (
		pub SKEY,
		("skey", "dictionary", "passphrase", "pronounceable", "memorable", "encoding"),
		("skey"),
		{ glyphs::SKEY_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));




#[ cfg (feature = "zt-patterns-pgp") ]
define_sequence! (
		pub PGP_TUPLE,
		(),
		(),
		[
			glyphs::PGP_EVEN_WORD_TOKEN,
			glyphs::PGP_ODD_WORD_TOKEN,
		], separators::SPACE_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-pgp") ]
define_repeat! (
		pub PGP,
		("pgp", "dictionary", "pronounceable", "encoding"),
		("pgp"),
		{ PGP_TUPLE => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(16 : 1));




#[ cfg (feature = "zt-patterns-eff-large") ]
define_repeat! (
		pub EFF_LARGE,
		("eff-large", "eff", "dictionary", "passphrase", "pronounceable", "memorable"),
		("eff-large"),
		{ glyphs::EFF_LARGE_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));

#[ cfg (feature = "zt-patterns-eff-short") ]
define_repeat! (
		pub EFF_SHORT,
		("eff-short", "eff", "dictionary", "passphrase", "pronounceable", "memorable"),
		("eff-short"),
		{ glyphs::EFF_SHORT_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));

#[ cfg (feature = "zt-patterns-eff-unique") ]
define_repeat! (
		pub EFF_UNIQUE,
		("eff-unique", "eff", "dictionary", "passphrase", "pronounceable", "memorable"),
		("eff-unique"),
		{ glyphs::EFF_UNIQUE_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(30 : 1));








#[ cfg (feature = "zt-patterns-pets-medium") ]
define_named! (
		pub PETS_MEDIUM_1,
		("pets-medium-1", "pets", "dictionary", "pronounceable", "memorable"),
		("pets-medium-1"),
		glyphs::PETS_MEDIUM_NAME_TOKEN);


#[ cfg (feature = "zt-patterns-pets-medium") ]
define_sequence! (
		pub PETS_MEDIUM_2,
		("pets-medium-2", "pets", "dictionary", "pronounceable", "memorable"),
		("pets-medium-2"), [
			glyphs::PETS_MEDIUM_ADJECTIVE_TOKEN,
			glyphs::PETS_MEDIUM_NAME_TOKEN,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);


#[ cfg (feature = "zt-patterns-pets-medium") ]
define_sequence! (
		pub PETS_MEDIUM_3,
		("pets-medium-3", "pets", "dictionary", "pronounceable", "memorable"),
		("pets-medium-3"), [
			glyphs::PETS_MEDIUM_ADVERB_TOKEN,
			glyphs::PETS_MEDIUM_ADJECTIVE_TOKEN,
			glyphs::PETS_MEDIUM_NAME_TOKEN,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);


#[ cfg (feature = "zt-patterns-pets-medium") ]
define_sequence! (
		pub PETS_MEDIUM_4,
		("pets-medium-4", "pets", "dictionary", "pronounceable", "memorable"),
		("pets-medium-4"), [
			glyphs::PETS_MEDIUM_ADVERB_TOKEN,
			glyphs::PETS_MEDIUM_ADJECTIVE_TOKEN,
			glyphs::PETS_COMMON_COLOR_TOKEN,
			glyphs::PETS_MEDIUM_NAME_TOKEN,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);


#[ cfg (feature = "zt-patterns-pets-medium") ]
define_all! (pub PETS_MEDIUM_ALL, [
		PETS_MEDIUM_1,
		PETS_MEDIUM_2,
		PETS_MEDIUM_3,
		PETS_MEDIUM_4,
	]);




#[ cfg (feature = "zt-patterns-pets-small") ]
define_named! (
		pub PETS_SMALL_1,
		("pets-small-1", "pets", "dictionary", "pronounceable", "memorable"),
		("pets-small-1", "pets-1"),
		glyphs::PETS_SMALL_NAME_TOKEN);


#[ cfg (feature = "zt-patterns-pets-small") ]
define_sequence! (
		pub PETS_SMALL_2,
		("pets-small-2", "pets", "dictionary", "pronounceable", "memorable"),
		("pets-small-2", "pets-2", "pets"), [
			glyphs::PETS_SMALL_ADJECTIVE_TOKEN,
			glyphs::PETS_SMALL_NAME_TOKEN,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);


#[ cfg (feature = "zt-patterns-pets-small") ]
define_sequence! (
		pub PETS_SMALL_3,
		("pets-small-3", "pets", "dictionary", "pronounceable", "memorable"),
		("pets-small-3", "pets-3"), [
			glyphs::PETS_SMALL_ADVERB_TOKEN,
			glyphs::PETS_SMALL_ADJECTIVE_TOKEN,
			glyphs::PETS_SMALL_NAME_TOKEN,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);


#[ cfg (feature = "zt-patterns-pets-small") ]
define_sequence! (
		pub PETS_SMALL_4,
		("pets-small-4", "pets", "dictionary", "pronounceable", "memorable"),
		("pets-small-4", "pets-4"), [
			glyphs::PETS_SMALL_ADVERB_TOKEN,
			glyphs::PETS_SMALL_ADJECTIVE_TOKEN,
			glyphs::PETS_COMMON_COLOR_TOKEN,
			glyphs::PETS_SMALL_NAME_TOKEN,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);


#[ cfg (feature = "zt-patterns-pets-small") ]
define_all! (pub PETS_SMALL_ALL, [
		PETS_SMALL_1,
		PETS_SMALL_2,
		PETS_SMALL_3,
		PETS_SMALL_4,
	]);








#[ cfg (feature = "zt-patterns-nato") ]
define_repeat! (
		pub NATO,
		("nato", "dictionary", "pronounceable"),
		("nato"),
		{ glyphs::NATO_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN },
		(66 : 1));








#[ cfg (feature = "zt-patterns-tokens") ]
define_named! (
		pub TOKEN_HEX_16,
		("token-hex-16", "tokens"),
		("token-hex-16", "x-16"),
		glyphs::INTEGER_16B_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_named! (
		pub TOKEN_HEX_24,
		("token-hex-24", "tokens"),
		("token-hex-24", "x-24"),
		glyphs::INTEGER_24B_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_named! (
		pub TOKEN_HEX_32,
		("token-hex-32", "tokens"),
		("token-hex-32", "x-32"),
		glyphs::INTEGER_32B_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_named! (
		pub TOKEN_HEX_48,
		("token-hex-48", "tokens"),
		("token-hex-48", "x-48"),
		glyphs::INTEGER_48B_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_named! (
		pub TOKEN_HEX_64,
		("token-hex-64", "tokens", "password"),
		("token-hex-64", "x-64"),
		glyphs::INTEGER_64B_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_named! (
		pub TOKEN_HEX_96,
		("token-hex-96", "tokens", "password"),
		("token-hex-96", "x-96"),
		glyphs::INTEGER_96B_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_named! (
		pub TOKEN_HEX_128,
		("token-hex-128", "tokens", "password"),
		("token-hex-128", "x-128"),
		glyphs::INTEGER_128B_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_sequence! (
		pub TOKEN_HEX_256,
		("token-hex-256", "tokens"),
		("token-hex-256", "x-256"), [
			glyphs::INTEGER_128B_HEX_TOKEN,
			glyphs::INTEGER_128B_HEX_TOKEN,
		], separators::HYPHEN_OPTIONAL_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_sequence! (
		pub TOKEN_HEX_512,
		("token-hex-512", "tokens"),
		("token-hex-512", "x-512"), [
			glyphs::INTEGER_128B_HEX_TOKEN,
			glyphs::INTEGER_128B_HEX_TOKEN,
			glyphs::INTEGER_128B_HEX_TOKEN,
			glyphs::INTEGER_128B_HEX_TOKEN,
		], separators::HYPHEN_OPTIONAL_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-tokens") ]
define_all! (pub TOKEN_ALL, [
		TOKEN_HEX_16,
		TOKEN_HEX_24,
		TOKEN_HEX_32,
		TOKEN_HEX_48,
		TOKEN_HEX_64,
		TOKEN_HEX_96,
		TOKEN_HEX_128,
		TOKEN_HEX_256,
		TOKEN_HEX_512,
	]);








#[ cfg (feature = "zt-patterns-uuid") ]
define_sequence! (
		pub UUID_V4,
		("uuid-v4", "uuid", "tokens", "password"),
		("uuid-v4"), [
			glyphs::UUID_ANY_FIELD_1_TOKEN,
			glyphs::UUID_ANY_FIELD_2_TOKEN,
			glyphs::UUID_V4_FIELD_3_TOKEN,
			glyphs::UUID_V4_FIELD_4_TOKEN,
			glyphs::UUID_ANY_FIELD_5_TOKEN,
		], separators::HYPHEN_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-uuid") ]
define_all! (pub UUID_ALL, [
		UUID_V4,
	]);








#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_127_PREFIX, Str, "127");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_10_PREFIX, Str, "10");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_172_PREFIX, Str, "172");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_192_A_PREFIX, Str, "192");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_192_B_PREFIX, Str, "168");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_169_A_PREFIX, Str, "169");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_169_B_PREFIX, Str, "254");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_100_A_PREFIX, Str, "100");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_198_A_PREFIX, Str, "198");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_MAC_02_PREFIX, Str, "02");
#[ cfg (feature = "zt-patterns-ip") ]
define_constant! (IP_MAC_04_PREFIX, Str, "04");

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_127,
		("ip-127", "ip", "networking"),
		("ip-127"),
		[
			IP_127_PREFIX_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_10,
		("ip-10", "ip", "networking"),
		("ip-10"),
		[
			IP_10_PREFIX_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_172,
		("ip-172", "ip", "networking"),
		("ip-172"),
		[
			IP_172_PREFIX_TOKEN,
			glyphs::INTEGER_1_30_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_192,
		("ip-192", "ip", "networking"),
		("ip-192"),
		[
			IP_192_A_PREFIX_TOKEN,
			IP_192_B_PREFIX_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_169,
		("ip-169", "ip", "networking"),
		("ip-169"),
		[
			IP_169_A_PREFIX_TOKEN,
			IP_169_B_PREFIX_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_100,
		("ip-100", "ip", "networking"),
		("ip-100"),
		[
			IP_100_A_PREFIX_TOKEN,
			glyphs::INTEGER_64_127_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_198,
		("ip-198", "ip", "networking"),
		("ip-198"),
		[
			IP_198_A_PREFIX_TOKEN,
			glyphs::INTEGER_18_19_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
			glyphs::INTEGER_2_253_TOKEN,
		], separators::DOT_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_MAC_02,
		("ip-mac-02", "ip-mac", "networking"),
		("ip-mac-02", "ip-mac"),
		[
			IP_MAC_02_PREFIX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
		], separators::COLON_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_sequence! (
		pub IP_MAC_04,
		("ip-mac-04", "ip-mac", "networking"),
		("ip-mac-04"),
		[
			IP_MAC_04_PREFIX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
			glyphs::INTEGER_8B_HEX_TOKEN,
		], separators::COLON_MANDATORY_INFIX_PATTERN);

#[ cfg (feature = "zt-patterns-ip") ]
define_all! (pub IP_ALL, [
		IP_127,
		IP_10,
		IP_172,
		IP_192,
		IP_169,
		IP_100,
		IP_198,
		IP_MAC_02,
		IP_MAC_04,
	]);








#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_STRING_DATETIME,
		("timestamp-date-time", "timestamp"),
		("timestamp-date-time", "date-time"),
		glyphs::TIMESTAMP_STRING_DATETIME_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_STRING_DATE,
		("timestamp-date", "timestamp"),
		("timestamp-date", "date"),
		glyphs::TIMESTAMP_STRING_DATE_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_STRING_TIME,
		("timestamp-time", "timestamp"),
		("timestamp-time", "time"),
		glyphs::TIMESTAMP_STRING_TIME_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_SECONDS_DEC,
		("timestamp-sec", "timestamp"),
		("timestamp-sec", "timestamp"),
		glyphs::TIMESTAMP_SECONDS_DEC_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_SECONDS_HEX,
		("timestamp-sec-hex", "timestamp"),
		("timestamp-sec-hex"),
		glyphs::TIMESTAMP_SECONDS_HEX_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_NANOSECONDS_DEC,
		("timestamp-nano", "timestamp"),
		("timestamp-nano"),
		glyphs::TIMESTAMP_NANOSECONDS_DEC_TOKEN);

#[ cfg (feature = "zt-patterns-timestamp") ]
define_named! (
		pub TIMESTAMP_NANOSECONDS_HEX,
		("timestamp-nano-hex", "timestamp"),
		("timestamp-nano-hex"),
		glyphs::TIMESTAMP_NANOSECONDS_HEX_TOKEN);


#[ cfg (any (feature = "zt-patterns-timestamp", feature = "zt-patterns-flake")) ]
define_named! (
		pub TIMESTAMP_FLAKE_SECONDS_DEC,
		("timestamp-flake", "timestamp"),
		("timestamp-flake"),
		glyphs::TIMESTAMP_FLAKE_SECONDS_DEC_TOKEN);

#[ cfg (any (feature = "zt-patterns-timestamp", feature = "zt-patterns-flake")) ]
define_named! (
		pub TIMESTAMP_FLAKE_SECONDS_HEX,
		("timestamp-flake-hex", "timestamp"),
		("timestamp-flake-hex"),
		glyphs::TIMESTAMP_FLAKE_SECONDS_HEX_TOKEN);


#[ cfg (feature = "zt-patterns-timestamp") ]
define_all! (pub TIMESTAMP_ALL, [
		TIMESTAMP_STRING_DATETIME,
		TIMESTAMP_STRING_DATE,
		TIMESTAMP_STRING_TIME,
		TIMESTAMP_SECONDS_DEC,
		TIMESTAMP_SECONDS_HEX,
		TIMESTAMP_NANOSECONDS_DEC,
		TIMESTAMP_NANOSECONDS_HEX,
		TIMESTAMP_FLAKE_SECONDS_DEC,
		TIMESTAMP_FLAKE_SECONDS_HEX,
	]);


#[ cfg (feature = "zt-patterns-flake") ]
define_repeat! (
		pub FLAKE_SECONDS,
		("flake", "timestamp"),
		("flake"),
		{
			( glyphs::TIMESTAMP_FLAKE_SECONDS_HEX_TOKEN, separators::HYPHEN_OPTIONAL_TOKEN, ),
			( glyphs::BYTES_HEX_4_TOKEN => separators::HYPHEN_OPTIONAL_INFIX_PATTERN ),
			()
		},
		(16 : 1, number_times_four_in_bits));








#[ cfg (feature = "zt-patterns-digits-base10") ]
define_permutation! (
		pub DIGITS_BASE10_PERMUTATION,
		("digits-base10-perm", "digits-base10", "permutation", "encoding", "digits", "password", "pronounceable", "memorable"),
		("digits-base10-perm", "dp"),
		glyphs::DIGIT_BASE10_TOKEN_PATTERNS,
		separators::SPACE_OPTIONAL_INFIX_EACH_5_PATTERN);

#[ cfg (feature = "zt-patterns-digits-base10") ]
define_all! (pub DIGITS_BASE10_PERMUTATION_ALL, [
		DIGITS_BASE10_PERMUTATION,
	]);




#[ cfg (feature = "zt-patterns-ascii") ]
define_permutation! (
		pub ASCII_LETTER_LOWER_PERMUTATION,
		("ascii-lower-perm", "ascii-lower", "permutation", "letters", "password"),
		("ascii-lower-perm"),
		glyphs::ASCII_LETTER_LOWER_TOKEN_PATTERNS,
		separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN);

#[ cfg (feature = "zt-patterns-ascii") ]
define_permutation! (
		pub ASCII_LETTER_UPPER_PERMUTATION,
		("ascii-upper-perm", "ascii-upper", "permutation", "letters"),
		("ascii-upper-perm"),
		glyphs::ASCII_LETTER_UPPER_TOKEN_PATTERNS,
		separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN);

#[ cfg (feature = "zt-patterns-ascii") ]
define_permutation! (
		pub ASCII_LETTER_MIXED_PERMUTATION,
		("ascii-mixed-perm", "ascii-mixed", "permutation", "letters", "password"),
		("ascii-mixed-perm"),
		glyphs::ASCII_LETTER_MIXED_TOKEN_PATTERNS,
		separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN);

#[ cfg (feature = "zt-patterns-ascii") ]
define_all! (pub ASCII_LETTER_PERMUTATION_ALL, [
		ASCII_LETTER_LOWER_PERMUTATION,
		ASCII_LETTER_UPPER_PERMUTATION,
		ASCII_LETTER_MIXED_PERMUTATION,
	]);








#[ cfg (feature = "zt-patterns-digits-base10") ]
define_permutation_partial! (
		pub DIGITS_BASE10_SHUFFLE,
		("digits-base10-shuf", "digits-base10", "shuffle", "encoding", "digits", "password", "pronounceable", "memorable"),
		("digits-base10-shuf", "ds"),
		{ glyphs::DIGIT_BASE10_TOKEN_PATTERNS => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(8 : 2));




#[ cfg (feature = "zt-patterns-ascii") ]
define_permutation_partial! (
		pub ASCII_LETTER_LOWER_SHUFFLE,
		("ascii-lower-shuf", "ascii-lower", "shuffle", "letters", "password"),
		("ascii-lower-shuf"),
		{ glyphs::ASCII_LETTER_LOWER_TOKEN_PATTERNS => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(24 : 4));

#[ cfg (feature = "zt-patterns-ascii") ]
define_permutation_partial! (
		pub ASCII_LETTER_UPPER_SHUFFLE,
		("ascii-upper-shuf", "ascii-upper", "shuffle", "letters"),
		("ascii-upper-shuf"),
		{ glyphs::ASCII_LETTER_UPPER_TOKEN_PATTERNS => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(24 : 4));

#[ cfg (feature = "zt-patterns-ascii") ]
define_permutation_partial! (
		pub ASCII_LETTER_MIXED_SHUFFLE,
		("ascii-mixed-shuf", "ascii-mixed", "shuffle", "letters", "password"),
		("ascii-mixed-shuf"),
		{ glyphs::ASCII_LETTER_MIXED_TOKEN_PATTERNS => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN },
		(48 : 4));








pub static ALL : &[&[Rb<TokenPattern>]] = &[
		
		#[ cfg (feature = "zt-patterns-digits-base10") ]
		DIGITS_BASE10_ALL,
		#[ cfg (feature = "zt-patterns-digits-base2") ]
		DIGITS_BASE2_ALL,
		#[ cfg (feature = "zt-patterns-digits-base8") ]
		DIGITS_BASE8_ALL,
		#[ cfg (feature = "zt-patterns-digits-base16") ]
		DIGITS_BASE16_ALL,
		#[ cfg (feature = "zt-patterns-digits-base32") ]
		DIGITS_BASE32_HEX_ALL,
		#[ cfg (feature = "zt-patterns-digits-base32") ]
		DIGITS_BASE32_RFC_ALL,
		#[ cfg (feature = "zt-patterns-digits-base64") ]
		DIGITS_BASE64_URL_ALL,
		#[ cfg (feature = "zt-patterns-digits-base64") ]
		DIGITS_BASE64_RFC_ALL,
		#[ cfg (feature = "zt-patterns-digits-base58") ]
		DIGITS_BASE58_ALL,
		#[ cfg (feature = "zt-patterns-digits-base62") ]
		DIGITS_BASE62_ALL,
		#[ cfg (feature = "zt-patterns-digits-bech32") ]
		DIGITS_BECH32_ALL,
		#[ cfg (feature = "zt-patterns-digits-z85") ]
		DIGITS_Z85_ALL,
		
		#[ cfg (feature = "zt-patterns-digits-base10") ]
		DIGITS_BASE10_PERMUTATION_ALL,
		#[ cfg (feature = "zt-patterns-digits-base10") ]
		DIGITS_BASE10_SHUFFLE_ALL,
		
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_LOWER_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_UPPER_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_MIXED_ALL,
		
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_SYMBOLS_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_PRINTABLE_ALL,
		
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_PERMUTATION_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_LOWER_SHUFFLE_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_UPPER_SHUFFLE_ALL,
		#[ cfg (feature = "zt-patterns-ascii") ]
		ASCII_LETTER_MIXED_SHUFFLE_ALL,
		
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_LOWER_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_UPPER_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_MIXED_ALL,
		
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_PLUS_A_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_PLUS_B_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		ASCII_CONSONANT_VOWEL_PLUS_C_ALL,
		
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_LOWER_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_UPPER_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_MIXED_ALL,
		
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_PLUS_A_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_PLUS_B_ALL,
		#[ cfg (feature = "zt-patterns-consonant-vowel") ]
		SIMPLE_CONSONANT_VOWEL_PLUS_C_ALL,
		
		#[ cfg (feature = "zt-patterns-apple") ]
		APPLE_ALL,
		
		#[ cfg (feature = "zt-patterns-proquint") ]
		PROQUINT_LOWER_ALL,
		#[ cfg (feature = "zt-patterns-proquint") ]
		PROQUINT_UPPER_ALL,
		
		#[ cfg (feature = "zt-patterns-koremutake") ]
		KOREMUTAKE_A_ALL,
		#[ cfg (feature = "zt-patterns-koremutake") ]
		KOREMUTAKE_B_ALL,
		
		#[ cfg (feature = "zt-patterns-mnemonic") ]
		MNEMONIC_ALL,
		#[ cfg (feature = "zt-patterns-bip0039") ]
		BIP0039_ALL,
		#[ cfg (feature = "zt-patterns-skey") ]
		SKEY_ALL,
		#[ cfg (feature = "zt-patterns-pgp") ]
		PGP_ALL,
		#[ cfg (feature = "zt-patterns-eff-large") ]
		EFF_LARGE_ALL,
		#[ cfg (feature = "zt-patterns-eff-short") ]
		EFF_SHORT_ALL,
		#[ cfg (feature = "zt-patterns-eff-unique") ]
		EFF_UNIQUE_ALL,
		
		#[ cfg (feature = "zt-patterns-pets-medium") ]
		PETS_MEDIUM_ALL,
		#[ cfg (feature = "zt-patterns-pets-small") ]
		PETS_SMALL_ALL,
		
		#[ cfg (feature = "zt-patterns-nato") ]
		NATO_ALL,
		
		#[ cfg (feature = "zt-patterns-bytes") ]
		BYTES_HEX_ALL,
		
		#[ cfg (feature = "zt-patterns-tokens") ]
		TOKEN_ALL,
		
		#[ cfg (feature = "zt-patterns-uuid") ]
		UUID_ALL,
		
		#[ cfg (feature = "zt-patterns-ip") ]
		IP_ALL,
		
		#[ cfg (feature = "zt-patterns-timestamp") ]
		TIMESTAMP_ALL,
		#[ cfg (feature = "zt-patterns-flake") ]
		FLAKE_SECONDS_ALL,
		
	];


