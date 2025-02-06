import os
import random
import string

def generate_random_utf8_file(size, filename):
    """
    Generates a random text file with the specified UTF-8 size.

    Parameters:
    size (int): The desired size of the file in UTF-8 characters.
    filename (str): The name of the file to be created.
    """

    # Random string generation using printable characters
    random_string = ''.join(random.choice(string.printable) for _ in range(size))

    # Writing to file
    with open(filename, "w", encoding="utf-8") as file:
        file.write(random_string)

# File sizes to generate
sizes = [1, 10, 100, 1000, 5000]
files = []

# Generate files and save to a list for downloading
for size in sizes:
    filename = f"random_{size}_chars.txt"
    generate_random_utf8_file(size, filename)
    files.append(filename)

files

