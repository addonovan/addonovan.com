head
    meta(charset: "UTF-8")
    title < "#[alias title] | addonovan"

    meta
        name: "viewport"
        content: "width=device-width, initial-scale=1.0"

    link
        rel: "stylesheet"
        href: "/highlighting/idea.css"

    script(src: "/highlighting/highlight.js")

body
    #[frag nav]

    div
        div.content.page
            h1 < "#[title]"

        #[content]

        div
            p.fglh.footer
                "Copyright &copy; 2016 Austin Donovan. All Rights Reserved."