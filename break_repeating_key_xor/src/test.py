
import base64

filename = "/home/casper/Crypto/crypto_pals/break_repeating_key_xor/src/6.txt"

with open(filename, "r") as f:
    for line in f:
        line = line.strip()
        decoded_line = base64.b64decode(line)
        print(len(decoded_line))
        # hex_line = decoded_line.hex()
        # print(hex_line)
        # print("Len: " + str(len(hex_line)))
        # print("\n")
