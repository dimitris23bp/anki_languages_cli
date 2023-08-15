# Anki language CLI

## Overview
This is a project dedicated to auto-translate word/phrases in any combination of languages and insert them in a deck inside desktop app of [Anki](https://apps.ankiweb.net/).

## Requirements
Anki should be installed and launched. In addition, Anki-Connect should also be installed. 
Take a look in [here](https://foosoft.net/projects/anki-connect/index.html#deck-actions) for more info. 

Unfortunately, the API is not official, thus the workaround of constantly running Anki with Anki-Connect plugin, is inevitable.

## How to run
In order to execute this project you can run the following line on your terminal:

```
cargo run -- -n <insert_word> -d <insert_deck> --source <source_lang> --target <target_lang>
```

Most of the arguments can be skipped. For more information, please run:

```
cargo run -- -h 
```


## Left to do
- Make it an executable, so that it will be possible to use without _cargo_
- Add more translation APIs and the option to choose between them 
- Add more tests to insure the robustness of the project
- Add more general commands (e.g. delete a deck, or even display word and study them) to transform this into a CLI anki