#!/usr/bin/python
# -*- coding: utf-8 -*-
import subprocess

# Define the file names
text_file1 = "input.txt"
text_file2 = "input.txt"

# Define the command to run script1.py with text_file1 piped in
script1_command = ["python3", "entity.py"]
with open(text_file1, "r") as f:
    result1 = subprocess.run(script1_command, stdin=f)

# Define the command to run script2.py with text_file2 piped in
script2_command = ["python3", "triagram.py"]
with open(text_file2, "r") as f:
    result2 = subprocess.run(script2_command, stdin=f)

# Print the output of the two scripts
print(result1)
print(result2)
