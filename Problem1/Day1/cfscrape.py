#!/bin/python3

# Description : This script scrapes a codeforces problem and writes it to files in1 in2 .. cor1 cor2 .. 
# Usage : ./cfscrape.py (URL to problem) -- print to output but not place files
# Usage : ./cfscrape.py (URL to problem) (Path to place files) -- print to output and place files

import sys    
import os
import requests
from bs4 import BeautifulSoup

def get_input_output(url):
    page = requests.get(url)
    soup = BeautifulSoup(page.content, "html.parser")
    example = soup.find("div", {"class": "sample-test"})

    inputs = example.find_all("div", {"class" : "input"})
    input_texts = []
    for inp in inputs:
        inp_text = ""
        for line in inp.find_all("div", {"class" : "test-example-line"}):
            inp_text += (line.text + "\n")
        input_texts.append(inp_text)
        print("Input")
        print(inp_text)

    outputs = example.find_all("div", {"class" : "output"})
    output_texts = []
    for out in outputs:
        out_text = out.find("pre").text
        output_texts.append(out_text)
        print("Output")
        print(out_text)

    ans = [input_texts, output_texts]
    return ans

def write_input_output(input_output_texts, rel_dest_dir="./"):
    output_path = os.path.join(os.getcwd(), rel_dest_dir)

    if not os.path.exists(output_path):
        os.makedirs(output_path)
    
    for idx, text in enumerate(input_output_texts[0]):
        file = open(os.path.join(output_path, "in" + str(idx)), "w")
        file.write(text)
        file.close()

    for idx, text in enumerate(input_output_texts[1]):
        file = open(os.path.join(output_path, "cor" + str(idx)), "w")
        file.write(text)
        file.close()

def get_problem_title(url):
    page = requests.get(url)
    soup = BeautifulSoup(page.content, "html.parser")
    title = soup.select_one('.header .title').text
    return title

args = len(sys.argv)

if args >= 2:
    url = sys.argv[1]
    data = get_input_output(url)

    if args >= 3:
        write_input_output(data, sys.argv[2])
