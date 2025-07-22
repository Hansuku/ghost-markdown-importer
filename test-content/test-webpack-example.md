---
title: 'webpack3.x初始配置'
date: 2017-11-04 10:51:08
tags: ['seo', 'VUE']
draft: false
summary: 'webPack 可以看做是模块打包机：它做的事情是，分析你的项目结构，找到 JavaScript 模块以及其它的一些浏览器不能直接运行的拓展语言（Scss，TypeScript 等），并将其打包为合适的格式以供浏览器使...'
images: ['https://www.hansuku.com/wp-content/themes/Lover/images/thumbs/1.jpg']
authors: ['default']
layout: PostLayout
---

# webpack3.x初始配置

这是一个关于webpack3.x配置的详细教程。

## 什么是webpack

webpack 可以看做是模块打包机：它做的事情是，分析你的项目结构，找到 JavaScript 模块以及其它的一些浏览器不能直接运行的拓展语言（Scss，TypeScript 等），并将其打包为合适的格式以供浏览器使用。

## 核心概念

- **Entry**: 入口起点
- **Output**: 输出配置
- **Loaders**: 模块转换器
- **Plugins**: 插件扩展

## 示例配置

```javascript
const path = require('path');

module.exports = {
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        use: 'babel-loader'
      }
    ]
  }
};
```