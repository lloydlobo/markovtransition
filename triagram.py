import random
import sys
from collections import defaultdict
from pprint import pprint

"""
$ cat input.txt | python3 triagram.py
"""


def main():
    # Read input text from stdin and split it into words.
    text = sys.stdin.read()
    words = text.split()
    # sentence = ["Alice", "was"]
    first_second_words = (words[0], words[1])

    # Generate trigrams from teh words.
    trigrams = [(words[i], words[i + 1], words[i + 2])
                for i in range(len(words) - 2)]

    """
    Build a dictionary where the keys are the first two words of each trigrams, 
    and the values are the third word of each trigram.
    # model = {}
    """
    # Build trigram model using a defaultdict.
    model = defaultdict(list)
    for w1, w2, w3 in trigrams:
        model[(w1, w2)].append(w3)

    """
    Use a generator expression instead of a list comprehension to generate the trigrams. 
    The current implementation generates a list of all the trigrams in the input text 
    using a list comprehension. This can be memory-intensive for large texts. Instead, 
    we can use a generator expression to generate the trigrams on the fly, which can save memory.
    """
    # Generate a sentence using the triagram model.
    sentence = [first_second_words[0], first_second_words[1]]
    while True:
        key = tuple(sentence[-2:])
        if key not in model:
            break
        next_word = random.choice(model[key])
        sentence.append(next_word)
        pass

    # Print the input text and generated text.
    # print("input:\n", " ".join(words))
    # print("model:")
    # pprint(dict(model))
    # print("Extracted features:")

    return " ".join(sentence)


if __name__ == "__main__":
    data = main()
    print(f"[output ]\n{data}")


"""
Alice was beginning to get very tired of sitting by her sister on the bank, 
and of having nothing to do: once or twice she had peeped into the book her 
sister was reading, but it had no pictures or conversations in it, 
'and what is the use of a book,' thought Alice 'without pictures or conversations?'

Alice was beginning to get very tired of sitting by her sister was reading, 
but it had no pictures or conversations?'
"""

"""
I like cats.
Cats are furry.
Dogs are friendly.

        I   like  cats  are   furry dogs  friendly
START   1   0     0     0     0     0     0
I       0   0.5   0.5   0     0     0     0
like    0   0     0     1     0     0     0
cats    0   0     0     0.5   0.5   0     0
are     0   0     0     0     0     0.5   0.5
furry   0   0     0     0     0     0     1
dogs    0   0     0     0     0     1     0
friendly0   0   0     0     0     0     0     1

I like cats are furry.
"""

"""
Use a more efficient data structure for the trigram model. 
The current implementation uses a dictionary to store the trigram model, which can be slow for large texts.
Instead, we can use a defaultdict to store the model, which is faster and more memory-efficient.
A defaultdict is a subclass of the built-in dict class that provides a default value for a nonexistent key when accessed.
In our case, we can use a defaultdict with a list as the default value, so that we don't need to check if a key exists before appending a value to its list.
"""
