

package main




import (
		
		. "github.com/volution/z-tokens/exchange/lib"
		
		"fmt"
		"os"
	)








func main () () {
	
	_recipient := TEST_RECIPIENT_PRIVATE_BECH32
	_sender := TEST_SENDER_PUBLIC_BECH32
	_ssh_wrap_handle := TEST_SSH_WRAP_HANDLE_BECH32
	
	if true {
		fmt.Fprintf (os.Stderr, "[--]\n")
		Decrypt (_recipient, _sender, "", "", TEST_ENCRYPTED_BOTH, "")
		fmt.Fprintf (os.Stderr, "[--]\n")
	}
	
	if true {
		fmt.Fprintf (os.Stderr, "[--]\n")
		Decrypt (_recipient, _sender, "", TEST_PIN_STRING, TEST_ENCRYPTED_BOTH_WITH_PIN, "")
		fmt.Fprintf (os.Stderr, "[--]\n")
	}
	
	if true {
		fmt.Fprintf (os.Stderr, "[--]\n")
		Decrypt (_recipient, _sender, TEST_SECRET_BECH32, TEST_PIN_STRING, TEST_ENCRYPTED_BOTH_WITH_PIN_AND_SECRET, "")
		fmt.Fprintf (os.Stderr, "[--]\n")
	}
	
	if true {
		fmt.Fprintf (os.Stderr, "[--]\n")
		Decrypt (_recipient, _sender, "", "", TEST_ENCRYPTED_BOTH_WITH_SSH_WRAP, _ssh_wrap_handle)
		fmt.Fprintf (os.Stderr, "[--]\n")
	}
}








var TEST_RECIPIENT_PRIVATE_BECH32 = "ztxrk1rp0qkrrrht77nh42pkzcf70uy3yrs5uxpq6uvql55h3jsgsxm9fqn6lxmk"
var TEST_SENDER_PUBLIC_BECH32 = "ztxsp1m6f9fwz0ukd7agd3udrlqsu8j0ltc8evfpzw7n040yjatkr4u4nq38he88"

var TEST_SECRET_BECH32 = "ztxcs1qvjhy8ftc7fjajtky3mcrgxdlacer2m6sj8hyxcaa2segdcnhjnqj7ylhm"

var TEST_PIN_STRING = "1234"

var TEST_SSH_WRAP_HANDLE_BECH32 = "ztxws1qqqqqzmnwd5z6etyxg6n2vfeqqqqqg9e9nt4vgd7c6s4tgmecj86kwpfpj7nsnnujzq6f4243jl2vecyrsqszf8qds8"




var TEST_ENCRYPTED_BOTH = []byte {
		0x89, 0xBB, 0x2C, 0x5C, 0xBB, 0x28, 0xAD, 0x49, 0x22, 0x32, 0xA5, 0x99, 0x48, 0x4C, 0x84, 0x44, 0x56, 0xB3, 0xFD, 0x3A, 0xEF, 0x2A, 0x93, 0xB5, 0x0F, 0x4F, 0x16, 0xDA, 0x8C, 0x8B, 0x25, 0x7A, 0x00, 0x88, 0x6F, 0x18, 0x0B, 0x91,
		0x23, 0x8C, 0x21, 0x8C, 0x0F, 0x1F, 0x76, 0xD0, 0xF4, 0xEB, 0xD2, 0xB0, 0x14, 0xCD, 0xDF, 0x43, 0x24, 0x55, 0x5C, 0x98, 0x92, 0x73, 0xA0, 0xA0, 0xBD, 0xF5, 0x05, 0x81, 0x3A, 0x8F, 0x02, 0x22, 0x40, 0x49, 0x18, 0xA6, 0xCB, 0x3C,
		0xCB, 0x49, 0x6E, 0xF1, 0xEA, 0x8D, 0x95, 0x12, 0xB8, 0x8F, 0x5C, 0xC0, 0x0E, 0x80, 0xE5, 0x16, 0x9B, 0xFB, 0x72, 0x81, 0xF1, 0x59, 0x48, 0x4E, 0x28, 0xCF, 0xA6, 0x02, 0xD0, 0xBE, 0x2A, 0x53, 0xF4, 0xCA, 0x7C, 0x94, 0x43, 0x78,
		0x27, 0x6E, 0x1E, 0x3E, 0xB6, 0x27, 0xCF, 0xA8, 0x65, 0xF3, 0x08, 0xA1, 0x62, 0x9D, 0x8C, 0xE4, 0x29, 0x6F, 0xDB, 0x19, 0x02, 0x44, 0x92, 0x22, 0x19, 0xF1, 0xCA, 0x49, 0x62, 0x5C, 0x63, 0xAE, 0xBE, 0x28, 0x60, 0xDF, 0xAB, 0xC3,
		0xCE, 0xAC, 0x9E, 0x58, 0x81, 0xB4, 0xF1, 0x1E, 0xB9, 0xF1, 0x9E, 0xF6, 0xDE, 0xDC, 0xC6, 0x3A, 0x9A, 0xFA, 0x72, 0xD7, 0x4B, 0x03, 0x61, 0x55, 0x18, 0x0D, 0x69, 0x4B, 0x1A, 0xA7, 0xB0, 0xEB, 0x80, 0x57, 0x66, 0x3E, 0x8D, 0xC3,
		0x7D, 0x18, 0x50, 0x6D, 0x87, 0x66, 0xC8, 0xD3, 0xA3, 0x28, 0xD7, 0x97, 0x93, 0xCD, 0x6B, 0xB9, 0x78, 0xDC, 0x37, 0xB8, 0x3D, 0x9E, 0x23, 0x47, 0xE2, 0x4F, 0x31, 0xC4, 0x65, 0x3A, 0x43, 0x1F, 0x81, 0x52, 0x8A, 0x34, 0x79, 0x95,
		0xCE, 0x60, 0xF1, 0xCA, 0x9B, 0xBB, 0x6A, 0xC5, 0x1B, 0x3F, 0xD9, 0x93, 0x6D, 0x31, 0x40, 0xC6, 0x9A, 0x53, 0x9B, 0xD1, 0xFC, 0xB4, 0x63, 0xB3, 0x9E, 0x8A, 0x14, 0x4E, 0xA9, 0x18, 0xBB, 0x94, 0x40, 0xA2, 0x67, 0xDE, 0x2A, 0xDA,
		0x0A, 0x3E, 0x4A, 0x47, 0x27, 0xD4, 0x4D, 0x85, 0x1E, 0x53, 0xFD, 0x1A, 0xD9, 0x00, 0x76, 0x16, 0x98, 0x8A, 0x25, 0xC9, 0x8F, 0x93, 0x8F, 0x02, 0xB2, 0x92, 0x22, 0x31, 0x1A, 0xC9, 0x1A, 0x91, 0x5D, 0xC7, 0xF8, 0x5E, 0xDF, 0x25,
		0x0D, 0x25, 0x85, 0xF9, 0xDE, 0xD1, 0x2B, 0x29, 0xD3, 0xF9, 0x54, 0x5C, 0xE2, 0xD5, 0xD5, 0xF6,
	}

var TEST_ENCRYPTED_BOTH_WITH_PIN = []byte {
		0xCE, 0x56, 0xED, 0xE0, 0x06, 0x8D, 0xA3, 0x5C, 0xC1, 0x9F, 0x3E, 0xA5, 0x31, 0xDF, 0x41, 0xE7, 0xCC, 0x98, 0x26, 0x04, 0x6E, 0x9D, 0x01, 0x3A, 0x7D, 0xEA, 0xB3, 0x3D, 0xEA, 0x7A, 0x87, 0xD1, 0x5B, 0xBA, 0xCB, 0x60, 0x0C, 0x32,
		0x09, 0x0A, 0x46, 0x1C, 0xB7, 0xFF, 0x7E, 0x27, 0x65, 0x77, 0x87, 0x81, 0x49, 0x15, 0xB9, 0x4E, 0x35, 0xFE, 0xA1, 0x94, 0xC9, 0xD3, 0xAE, 0x45, 0x10, 0xD0, 0x96, 0x04, 0x52, 0x21, 0x72, 0xF8, 0x24, 0x9F, 0x2C, 0x21, 0x4A, 0x3B,
		0x74, 0x12, 0x84, 0x6D, 0x3E, 0x04, 0x29, 0xA1, 0x22, 0x53, 0x4B, 0x22, 0xC2, 0x97, 0x02, 0xA4, 0x7D, 0x62, 0x0E, 0x31, 0x50, 0xCF, 0xC2, 0xED, 0xFE, 0x4E, 0x16, 0x97, 0xA6, 0xA6, 0x00, 0xB9, 0x44, 0xAB, 0x80, 0x59, 0x6C, 0xE1,
		0x04, 0x50, 0xC5, 0x26, 0x66, 0xAF, 0xB9, 0x1D, 0x22, 0xC0, 0x6E, 0x40, 0xA8, 0x7F, 0x88, 0xE1, 0xCD, 0xC4, 0x72, 0x16, 0x07, 0xBC, 0xD5, 0x0D, 0x24, 0x3B, 0x65, 0xBD, 0x3A, 0xA6, 0xAF, 0x03, 0x27, 0xBA, 0x68, 0xDE, 0x28, 0xB5,
		0xB9, 0xB6, 0x81, 0x27, 0x7C, 0x05, 0x1F, 0xFE, 0x5A, 0x70, 0x4D, 0xA0, 0xE7, 0x05, 0x50, 0xA3, 0xCB, 0x84, 0xAF, 0x62, 0x44, 0xFD, 0xCE, 0xF6, 0x46, 0x4F, 0x8B, 0xAE, 0x00, 0x1F, 0x6B, 0x3F, 0x93, 0x4C, 0x90, 0xB6, 0x0F, 0xE2,
		0x31, 0xF7, 0x26, 0x59, 0x61, 0x08, 0xB6, 0x25, 0x75, 0x2A, 0xF5, 0x7C, 0x22, 0x65, 0x04, 0x24, 0x16, 0x77, 0xF2, 0xA4, 0x18, 0x21, 0x99, 0xC1, 0xE4, 0xBD, 0xEB, 0xD3, 0xE0, 0x58, 0xBD, 0x1C, 0x87, 0xB5, 0x67, 0x6F, 0xCA, 0xF3,
		0xFA, 0xD5, 0xD4, 0xD7, 0xED, 0xF1, 0x49, 0xAB, 0x5C, 0xA9, 0x75, 0x85, 0xDD, 0x85, 0xBF, 0xA0, 0xA9, 0x97, 0xFD, 0x11, 0x55, 0xDD, 0xA7, 0x02, 0x12, 0xB3, 0x88, 0x57, 0x21, 0x70, 0xC2, 0x9E, 0x04, 0x66, 0x62, 0x62, 0x86, 0x8E,
		0xAB, 0x06, 0xDF, 0x47, 0x89, 0x52, 0xE0, 0xF3, 0x89, 0xFC, 0xCD, 0xFD, 0xDF, 0x44, 0xBE, 0x7B, 0xFC, 0x20, 0x5F, 0x88, 0x16, 0x50, 0x07, 0x7B, 0xDA, 0x26, 0x49, 0xD8, 0xB4, 0x93, 0xD4, 0xD3, 0x8B, 0x06, 0x28, 0x01, 0x84, 0x8E,
		0x3A, 0x08, 0x8A, 0x60, 0xA1, 0x15, 0x6B, 0x8B, 0xA5, 0x52, 0xC2, 0x9C, 0x3A, 0x58, 0xA3, 0xC3,
	}

var TEST_ENCRYPTED_BOTH_WITH_PIN_AND_SECRET = []byte {
		0x4D, 0xD4, 0x6B, 0x90, 0xCE, 0x93, 0x19, 0xA0, 0x08, 0xB6, 0xB5, 0xFB, 0x47, 0xD9, 0x6A, 0x79, 0x80, 0xFC, 0x32, 0xC2, 0x11, 0x1A, 0xA6, 0xDC, 0xD0, 0xF5, 0x5A, 0x31, 0xF7, 0x5E, 0x7C, 0x26, 0x4D, 0xA6, 0x3D, 0x0E, 0x47, 0xE7,
		0xE9, 0xE6, 0xE1, 0x5D, 0x85, 0x07, 0xBE, 0xD6, 0xDF, 0xDF, 0xCD, 0x5E, 0x52, 0xC9, 0x2F, 0x8E, 0x2B, 0x2C, 0xC4, 0x2F, 0xED, 0x33, 0x24, 0xAD, 0x5D, 0x33, 0x4B, 0xC9, 0x6C, 0x6D, 0x92, 0xAC, 0xDD, 0xB4, 0x40, 0x5A, 0xFC, 0x86,
		0x5B, 0x35, 0xA4, 0x45, 0x23, 0x1F, 0x3A, 0xA3, 0x3C, 0xC9, 0x75, 0xBC, 0x43, 0xAC, 0x12, 0x87, 0x42, 0x5D, 0x95, 0x4F, 0xD0, 0xBE, 0x2A, 0xD7, 0xC1, 0x1B, 0xCD, 0xAB, 0x77, 0x07, 0xDF, 0x66, 0xB6, 0xF6, 0x77, 0x51, 0xEC, 0xDE,
		0xAF, 0xBA, 0xBC, 0x79, 0x6C, 0x76, 0xD4, 0xD9, 0x3A, 0x9B, 0xCF, 0x8D, 0x76, 0x9F, 0xD5, 0x2D, 0x81, 0x32, 0x80, 0xC0, 0xDC, 0xA6, 0xB3, 0x41, 0xD1, 0x60, 0x88, 0x15, 0x4A, 0x14, 0x5D, 0x2F, 0x46, 0xD5, 0x9F, 0xDC, 0x32, 0xEF,
		0xE9, 0x2F, 0xDD, 0x98, 0xA8, 0x41, 0x51, 0x87, 0x74, 0x9D, 0x7A, 0xC7, 0xAF, 0x08, 0x77, 0x8A, 0x77, 0xA4, 0x02, 0x62, 0x32, 0x0A, 0x0A, 0x4F, 0xE5, 0xF7, 0x61, 0xE5, 0x71, 0xF3, 0x72, 0x31, 0xE7, 0x79, 0x6F, 0xCD, 0x26, 0x5D,
		0xE1, 0x27, 0x98, 0x5A, 0x7E, 0x9E, 0x90, 0xC6, 0x0A, 0xD4, 0xAB, 0xD3, 0x9D, 0x10, 0x99, 0x3B, 0x12, 0x6B, 0xC6, 0x6A, 0x4A, 0xC0, 0xE0, 0x48, 0xC1, 0x04, 0x10, 0x5E, 0x6C, 0x20, 0x06, 0x7C, 0x28, 0xDF, 0xAD, 0xB3, 0xD1, 0x2E,
		0x3C, 0xBB, 0xA4, 0xA0, 0xC3, 0xC3, 0x72, 0x00, 0xD4, 0xA9, 0xD1, 0xC5, 0xA1, 0xE8, 0x8F, 0x2F, 0x8E, 0x93, 0xE1, 0x19, 0x03, 0xD2, 0xCE, 0xBB, 0xA2, 0xC5, 0x8C, 0x87, 0x6F, 0x58, 0x55, 0xA7, 0xED, 0x3E, 0xB9, 0x5D, 0xFE, 0x91,
		0x22, 0x2F, 0x19, 0x14, 0x56, 0x64, 0x1E, 0x9E, 0xEC, 0x07, 0x8B, 0x76, 0xCC, 0xC3, 0x7B, 0xD3, 0xF9, 0xA6, 0xC8, 0x4D, 0x01, 0x7A, 0x3A, 0xCF, 0x75, 0x3E, 0x96, 0xFA, 0x8D, 0x91, 0xFD, 0xC6, 0x72, 0xF2, 0xC3, 0x33, 0x82, 0x86,
		0x5C, 0x79, 0xB0, 0xED, 0x18, 0xFF, 0xC9, 0x85, 0x73, 0x20, 0x0F, 0xFA, 0x50, 0x5E, 0x2D, 0xFB,
	}

var TEST_ENCRYPTED_BOTH_WITH_SSH_WRAP = []byte {
		0xD4, 0x81, 0xD8, 0x36, 0xA7, 0x29, 0xE1, 0x8D, 0x6D, 0x9F, 0x15, 0x3B, 0x68, 0xF4, 0x81, 0x09, 0xED, 0x2B, 0x5D, 0xFD, 0xB5, 0x32, 0x16, 0x41, 0xBA, 0xEE, 0x97, 0xFF, 0x41, 0x11, 0x06, 0x9F, 0xFF, 0xCC, 0x2F, 0xCB, 0x91, 0xF6,
		0xE2, 0x68, 0x84, 0xD5, 0xFA, 0xB3, 0x16, 0xD6, 0x62, 0xD2, 0xFE, 0xBD, 0x57, 0xED, 0x3F, 0xFC, 0x13, 0x9C, 0x03, 0x35, 0xF5, 0xA5, 0xC9, 0xB1, 0x62, 0xC2, 0x2B, 0xA0, 0x67, 0xDD, 0x3C, 0x20, 0x08, 0x9B, 0x91, 0x31, 0x09, 0xD3,
		0x85, 0x95, 0x94, 0x8E, 0x69, 0xC5, 0x79, 0xCA, 0xA9, 0x7A, 0x4B, 0x1B, 0x0F, 0x91, 0x68, 0xA0, 0x93, 0xFD, 0x39, 0x66, 0x51, 0x9A, 0x30, 0x58, 0xF3, 0xDC, 0x72, 0x58, 0xF0, 0xED, 0xDA, 0x8C, 0x80, 0xFD, 0x03, 0xA8, 0xB9, 0x39,
		0x41, 0x21, 0x32, 0x3E, 0x49, 0x0D, 0x82, 0x62, 0x06, 0xD2, 0x20, 0x2C, 0x2F, 0x3C, 0xA0, 0x30, 0x68, 0x13, 0x58, 0x71, 0xE5, 0x84, 0x8D, 0xDA, 0x5B, 0x0E, 0x8F, 0x98, 0x2B, 0xEC, 0x42, 0x68, 0x16, 0x02, 0xC6, 0x2C, 0x5E, 0x48,
		0x64, 0x89, 0x77, 0x59, 0x98, 0x49, 0x54, 0xA6, 0xDF, 0xB0, 0xF4, 0xB6, 0x80, 0x63, 0x6D, 0x31, 0x0B, 0x15, 0x1D, 0x8E, 0x79, 0xBB, 0xF0, 0x57, 0xB3, 0x55, 0x5C, 0x1D, 0x41, 0x8D, 0xE0, 0x18, 0xB6, 0xC3, 0x87, 0x0B, 0x13, 0x40,
		0xEE, 0xCB, 0x7D, 0x8D, 0xC6, 0x55, 0x02, 0x0A, 0xE9, 0xFB, 0x71, 0x45, 0x63, 0x33, 0xE4, 0x8D, 0x5B, 0x98, 0xD8, 0x2D, 0xFD, 0x7C, 0xB4, 0xDF, 0x4F, 0x81, 0x3B, 0xEE, 0x52, 0xC6, 0xC0, 0xB4, 0x4E, 0x84, 0xD2, 0x0F, 0x2B, 0x63,
		0xD9, 0x51, 0xF6, 0x0B, 0x37, 0xA0, 0x49, 0x1A, 0xE2, 0x12, 0x7D, 0x77, 0xBC, 0x13, 0x9B, 0x57, 0x92, 0x87, 0x04, 0x17, 0x26, 0x29, 0x18, 0x27, 0x9B, 0xCC, 0xCF, 0xF6, 0x47, 0x11, 0x7D, 0x32, 0xC6, 0xD9, 0x89, 0xF3, 0xA6, 0x29,
		0xBB, 0x05, 0xBF, 0xED, 0x86, 0xBC, 0xBB, 0x8A, 0x8E, 0xA3, 0x17, 0x6F, 0xFB, 0xF3, 0x0D, 0x3B, 0x95, 0x25, 0x54, 0x98, 0xC7, 0xC6, 0x84, 0x2C, 0xEE, 0xD3, 0x90, 0x77, 0xC0, 0xEA, 0xA7, 0x57, 0xB2, 0x5E, 0xB3, 0x77, 0x0F, 0x4E,
		0x37, 0xB5, 0xF3, 0x87, 0xDE, 0x0B, 0xE8, 0x83, 0x77, 0x14, 0x23, 0xCC, 0x8A, 0x97, 0x34, 0xB8,
	}


