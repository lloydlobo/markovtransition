import spacy
import torch
import transformers

nlp = spacy.load("en_core_web_sm")

text = (
    "When Sebastian Thrun started working on self-driving cars at "
    "Google in 2007, few people outside of the company took him "
    "seriously. “I can tell you very senior CEOs of major American "
    "car companies would shake my hand and turn away because I wasn’t "
    "worth talking to,” said Thrun, in an interview with Recode earlier "
    "this week."
)

doc = nlp(text)

# Convert the processed text to a tensor using PyTorch and Hugging Face Transformers
tokenizer = transformers.AutoTokenizer.from_pretrained("bert-base-uncased")
model = transformers.AutoModel.from_pretrained("bert-base-uncased")
inputs = tokenizer(text, return_tensors="pt")
outputs = model(**inputs)

# Print the output tensor
print(outputs)
