# How to Use Spectacle.js for writing slides in markdown

## Install yarn

First install yarn through your OS's favorite package manager. If you already have it on your machine then it is great you can jump into next section

For macOS

```bash
$ brew install yarn
```

For Debian
```bash
$ sudo apt-get install yarn
```

## Install the CLI tooling

Then you can install cli utility for creating slide projects with spectacle.js

```bash
$ yarn global add spectacle-cli
```

Then visit [here](https://github.com/FormidableLabs/spectacle-cli) for getting up to date information on how to generate boilerplates for creating slide decks with the tool !!

## Install the node modules

The repo doesn't come with the libraries you need to install them but it is easy, all you have to do is run this

```bash
$ yarn install
```

## Edit the slides

Slides are in the folder ```slide-decks/src/``` in a file named ```slides.md```

## Build && Start serving slides !!

One line command to serve 

```sh
$ yarn start

# Or with a specific port!
$ yarn start --port=3000
```

## Release them

Second one line command to build

```sh
$ yarn build
```

From there you can deploy the built slides  in `dist` to services like Netlify, Surge, etc!

> Some parts of this is taken from the documentation supplied with spectacle