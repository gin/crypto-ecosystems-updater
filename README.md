# Crypto Ecosystems Updater

The repository [electric-capital/crypto-ecosystems](https://github.com/electric-capital/crypto-ecosystems) gathers data about open source projects in the crypto ecosystem.
This program helps us update this dataset by automating some parts of it.

## Usage with stdin
1. curl GitHub API to pass the response to our program (using the organization `Stride-Labs` as example)
```
curl -L \                                    
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <YOUR_GITHUB_TOKEN>" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/orgs/Stride-Labs/repos
| ./crypto-ecosystems-updater
```

## Usage with existing JSON file
1. API call to GitHub to get JSON response (using the organization `Stride-Labs` as example)
```
curl -L \                                    
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <YOUR_GITHUB_TOKEN>" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/orgs/Stride-Labs/repos
```

2. Convert JSON to TOML
```
./crypto-ecosystems-updater <path_to_json_file>
```
