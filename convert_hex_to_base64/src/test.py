import binascii
import base64

def hex_to_base64(hex_string):
    # Convert hex string to binary
    binary_data = binascii.unhexlify(hex_string)
    # Encode binary data as base64
    base64_data = base64.b64encode(binary_data)
    # Convert base64 data to a string
    base64_string = base64_data.decode('utf-8')
    return base64_string


hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
base64_string = hex_to_base64(hex_string)
print(base64_string)