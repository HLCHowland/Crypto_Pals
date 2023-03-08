import binascii

def xor_single_char(byte_str, char):
    result = b''
    for byte in byte_str:
        result += bytes([byte ^ char])
    return result

def score_plaintext(plaintext):
    score = 0
    freq = {'E': 12.70, 'T': 9.06, 'A': 8.17, 'O': 7.51, 'I': 6.97, 'N': 6.75, 'S': 6.33, 'H': 6.09, 'R': 5.99, 'D': 4.25, 'L': 4.03, 'U': 2.76, 'C': 2.78, 'M': 2.41, 'F': 2.23, 'Y': 2.02, 'W': 2.36, 'G': 2.02, 'P': 1.93, 'B': 1.29, 'V': 0.98, 'K': 0.77, 'X': 0.15, 'Q': 0.10, 'J': 0.15, 'Z': 0.07}
    for char in plaintext.upper():
        if char in freq:
            score += freq[char]
    return score

def main():
    hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    byte_str = binascii.unhexlify(hex_str)
    max_score = 0
    plaintext = ''
    for char in range(256):
        candidate = xor_single_char(byte_str, char)
        candidate_score = score_plaintext(candidate.decode('utf-8'))
        if candidate_score > max_score:
            max_score = candidate_score
            plaintext = candidate.decode('utf-8')
    print(plaintext)

if __name__ == '__main__':
    main()