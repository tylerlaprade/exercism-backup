def is_pangram(sentence):
    lowerSentence = sentence.lower()
    for letter in "abcdefghijklmnopqrstuvwxyz":
        if (lowerSentence.find(letter) == -1):return False
    return True
