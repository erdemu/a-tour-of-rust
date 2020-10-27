# How to setup 

## Install pre-requisites

Install yarn as follows

```sh
# macOS
brew install yarn
# debian
apt-get install yarn
```

## Init a folder

You need a project folder for your slide deck

```sh
yarn init
```

Answer to the onscreen questions

## Add mdx-deck

Add the mdx-deck dep to your project

```sh
yarn add mdx-deck
```

## Create the slide text

```sh
touch slide_text.mdx
```

## Add a script start to your package.json file

add this to your json array

```json
  "scripts": {
    "start": "mdx-deck slide_text.mdx"
  }
```

## Run dev server

```sh
yarn start
```

## Edit your slides