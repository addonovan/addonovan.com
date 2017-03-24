head
    meta(charset: "UTF-8")
    title < "#[alias package]::#[alias title] | addonovan"

    meta
        name: "viewport"
        content: "width=device-width, initial-scale=1.0"

    link
        rel: "stylesheet"
        href: "/highlighting/idea.css"

    script(src: "/highlighting/highlight.js")

body
    p.whyisyourscreensotiny < "Why is your screen so narrow, dear lord?"

    #[frag nav]

    div
        h1.title.nbp < "#[title]"
        h2.subtitle.ntp < "#[package]"

        #[content]

        div
            p.fglh.footer
                "Copyright &copy; 2016 Austin Donovan. All Rights Reserved."