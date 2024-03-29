module.exports = {
    browser: ['firefox'],
    injectChanges: false,
    ghostMode: false,
    minify: false,
    notify: false,
    port: 3000,
    reloadOnRestart: true,
    server: {
        baseDir: 'site/build',
        index: 'index.html',
        serveStaticOptions: {
            extensions: ['html'],
        },
    },
    watch: true,
}
