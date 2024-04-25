<br />
<div align="center">
  <a href="https://github.com/ElmTran/tts_cli_rs">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">TTS-CLI-RS</h3>

  <p align="center">
    A simple CLI tool to convert text to speech based on Azure Cognitive Services, using Rust.
    <br />
    <a href="https://github.com/ElmTran/tts_cli_rs"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/ElmTran/tts_cli_rs/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/ElmTran/tts_cli_rs/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

## About The Project

[![TTS-CLI_RS][product-screenshot]](images/screenshot.png)

## Getting Started

### Prerequisites

1. Get an API Key and Endpoint from Azure Cognitive Services. You can get a resource from [here](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/get-started-text-to-speech?tabs=windows%2Cterminal&pivots=programming-language-csharp#prerequisites)

### Installation

- Download the latest release from the [releases page](https://github.com/ElmTran/tts_cli_rs/releases/)

- Build the project from source

  ```sh
  git clone https://github.com/ElmTran/tts_cli_rs.git

  cd tts_cli_rs

  cargo build --release
  ```

- Or you can fork this repo and build it by Github Actions, then download the artifact from the Release page.

## Usage

- Basic usage

  ```sh
  # Show help
  tts_cli_rs --help

  # Set the API Key
  tts_cli_rs config --set key=<API_KEY>

  # Set the Endpoint
  tts_cli_rs config --set endpoint=<ENDPOINT>

  # Convert text to speech
  tts_cli_rs --text "Hello, World!"

  # optional arguments:
  #   -l, --language <language>    The language of the input text. Default is "en-US".
  #   -p, --pitch <pitch>          The voice pitch. Default is 0.
  #   -r, --rate <rate>            The voice rate. Default is 1.
  #   -s, --speaker <speaker>      The speaker name. Default is "en-US-AriaNeural".
  #   -y, --style <style>          The speaking style. Default is "chat".
  #   file/dir                     The file or directory to convert.
  ```

  More languages, speakers, and styles can be found [here](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/language-support?tabs=stt#supported-languages)

- The TTS-CLI-RS can corporate with other tools, like AHK (AutoHotKey) to convert the selected text to speech.

  ```ahk
  ; Convert the selected text to speech
  ^+s::
  Send, ^c
  Run, tts_cli_rs.exe --text "%clipboard%"
  return
  ```

## Roadmap

- [ ] Create a GUI version

- [ ] Add more services, like Parler (local TTS service)

See the [open issues](https://github.com/ElmTran/tts_cli_rs/issues) for a full list of proposed features (and known issues).

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

[![License][license-shield]][license-url]

## Contact

Mengqing - [@Telegram](https://t.me/mzfbwu/) - [@Email](mailto:c897611977@gmail.com)

Project Link: [https://github.com/ElmTran/tts_cli_rs](https://github.com/ElmTran/tts_cli_rs)

## Acknowledgments

- [azure-cognitiveservices-speech](https://learn.microsoft.com/en-us/azure/ai-services/speech-service/get-started-text-to-speech?tabs=windows%2Cterminal&pivots=programming-language-rest#synthesize-to-a-file)
- [reqwest](https://docs.rs/reqwest/latest/reqwest/)
- [soloud](https://solhsa.com/soloud/index.html)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[license-shield]: https://img.shields.io/github/license/ElmTran/tts_cli_rs
[license-url]: https://github.com/ElmTran/tts_cli_rs/blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png
