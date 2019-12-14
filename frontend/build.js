var {TsConfigPathsPlugin} = require('awesome-typescript-loader')
var ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');


var webpack = require("webpack"),
    path = require("path");

let plugins = [
    new ForkTsCheckerWebpackPlugin({
        tsconfig: path.resolve(__dirname, "./tsconfig.json"),
        ignoreDiagnostics: [2307, 2371]
    }),
];

process.env.UV_THREADPOOL_SIZE = 4;

const watchIgnorePaths = [
    path.resolve(__dirname, '../../node_modules/'),
    path.resolve(__dirname, '../public/')
];

plugins.push(new webpack.WatchIgnorePlugin(watchIgnorePaths));

var compiler = webpack({
    mode: "development",

    module: {
        rules: [
            {
                test: /\.tsx?$/,
                exclude: /(node_modules|bower_components)/,
                loader: 'ts-loader',
                query: {
                    configFile: path.resolve(__dirname, "./tsconfig.json"),
                    silent: true,
                    transpileOnly: true,
                    ignoreDiagnostics: []
                }
            },
            {
                test: /\.scss/,
                use: ["style-loader", "css-loader", "sass-loader"]
            },
            {
                test: /\.less/,
                use: ["style-loader", "css-loader", "less-loader"]
            },
            {
                test: /\.css/,
                use: ["style-loader", "css-loader"]
            },
            {
                test: /\.(woff|gif|eot|ttf|svg)/,
                use: ["url-loader"]
            },
            {enforce: "pre", test: /\.js$/, exclude: /(node_modules|bower_components)/, loader: "source-map-loader"}
        ]
    },
    cache: true,
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".json"],
        plugins: [
            new TsConfigPathsPlugin()
        ]
    },
    entry: {
        "app": path.resolve(__dirname, "./src/app.tsx"),
    },
    output: {
        path: __dirname + "/public",
        filename: "[name]Bundle.js"
    },
    plugins: plugins
});

var runfn = function (err, stats) {
    console.log("time:", (stats.endTime - stats.startTime) / 1000)
    if (err) {
        console.log(err);
    } else {
        if (stats.compilation.errors) {
            console.log(stats.compilation.errors);
        }
    }
};


compiler.watch({
    aggregateTimeout: 300, // wait so long for more changes
    poll: true
}, runfn);