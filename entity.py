import sys

import spacy

"""
$ cat input.txt | python3 entity.py
"""

nlp = spacy.load("en_core_web_sm")
TEXT = """
Mary had a little lamb. 
The Lamb is cute. 
The little lamb loves her. 
She named the small little lamb Harry. 
The house they lived in was in London.
"""


# ('Mary', 'PERSON')
# ('Lamb', 'PERSON')
# ('London', 'GPE')


def get_stdin_args():
    import argparse

    parser = argparse.ArgumentParser(
        description="Parse text and extract it's named entities",
    )
    parser.add_argument("--text", metavar="N", type=str, nargs="+", help=f"Input text")
    args = parser.parse_args()
    return args


def extract_named_entities(text):
    corpus = nlp(text)
    if text is None:
        corpus = nlp(TEXT)
    entities = []
    # Extract named entities.
    for ent in corpus.ents:
        entities.append((ent.text, ent.label_))
    # Print named entities.
    for ent in entities:
        # print(ent)
        pass
    return entities


def get_sys_stdin():
    buf = []
    for line in sys.stdin:
        buf.append(line.strip())
    return buf


# $ cat input.txt | python3 entity.py
def main():
    buf = get_sys_stdin()
    text = " ".join(buf)
    entities = extract_named_entities(text)
    return entities


if __name__ == "__main__":
    data = main()
    print(data)
