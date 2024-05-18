import argparse
import requests
from html2text import html2text

# Create an argument parser
parser = argparse.ArgumentParser(description='Fetch markdown for a given challenge.')
parser.add_argument('challenge', type=str, help='The challenge for which to fetch markdown')
args = parser.parse_args()

# URL for the GraphQL endpoint
url = 'https://leetcode.com/graphql/'

# Headers for the POST request
headers = {
    'Content-Type': 'application/json',
}

# GraphQL query to fetch the challenge content
payload = {
    "query": """
    query questionContent($titleSlug: String!) {
        question(titleSlug: $titleSlug) {
            content
            mysqlSchemas
            dataSchemas
        }
    }
    """,
    "variables": {"titleSlug": args.challenge},
    "operationName": "questionContent"
}

# Send the POST request to the GraphQL endpoint
response = requests.post(url, json=payload, headers=headers)
response_data = response.json()

# Extract the HTML content from the response
html_content = response_data['data']['question']['content']

# Convert the HTML content to Markdown using html2text
markdown_content = html2text(html_content)

# Print the Markdown content
print(markdown_content)
