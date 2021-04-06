# douki-next

## <a name='Tableofcontents'></a>Table of contents

*  [Table of contents](#Tableofcontents)
*  [About douki](#Aboutdoki)
*  [Why douki?](#Whydoki)
*  [Roadmap](#Roadmap)
*  [References, inspirations](#Referencesinspirations)

## <a name='Aboutdoki'></a>About doki

Douki-next (同期『ネクスト』) is a synchronization software for [anilist](https://anilist.co) and [myanimelist](https://myanimelist.net). The purpose of douki is to help you updating at the same time your anilist and myanimelist with precision.
This project is still in its early stage of development —in fact, almost nothing works! So be careful experimenting with this program. Hopefully one day when some release will pop in the github release section, it will definitely be safe though.

## <a name='Whydoiki'></a> Why doki

- Meant for everyone, easy to use, yet sufficiently versatile to process complex sync.
- Available [everywhere Rust can compile](https://doc.rust-lang.org/rustc/platform-support.html), prebuilt are proposed in the release section for Windows, Linux and MacOS
- Douki is meant to be *safe*. Using Douki you could can cancel changes at any time while the program is still runing, errors are human so you should be able to cancel yours.


# <a name='Roadmap'></a> Roadmap

- Core
    - [x] Retrieve Anilist's list
    - [ ] Retrieve MAL's list
    - [ ] Mutate Anilist's list
    - [ ] Mutate MAL's list
    - [ ] Safe mutate (i.e. possibility to cancel anything while the program is still runing)
    - [ ] Choosing field
    - [ ] Mutate only if not already the same
- Advanced
    - [ ] favourite support (characters, animes, manga)


## <a name='Referencesinspirations'></a>References, inspirations

* Idea inspired by the late project also named [douki](https://github.com/gilmoreg/douki)

Please take a look at similar projects!

- [Taiga](https://taiga.moe) can update your Anilist and myanimelist while watching anime with their app
- [Malsync](https://malsync.moe) can update your Anilist, myanimelist, and others while watching anime on their supported website.