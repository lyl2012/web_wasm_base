# 简介
yew-0.19+tailwindcss3.0 基础模版，使用trunk进行编译管理

## 工具安装:
1. cargo install --locked trunk
2. rustup target add wasm32-unknown-unknown

## 静态资源
1. trunk 配置文件在static/Trunk.toml
2. css使用tailwindcss3.0
3. node版本为:v18.12 npm版本为：9.1.2

## 生成css
1. npm install
2. npm run build-css

## 启动服务
1. cd static
2. trunk serve

## PostCss安装tailwindcss步骤
1. 在项目根目录下，创建static目录. cd static
2. npm init -y
3. npm install tailwindcss@latest postcss@latest autoprefixer@latest postcss-cli@latest
4. 新建postcss.config.js,写入如下内容:
```
// postcss.config.js
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  }
}
```

5. npx tailwindcss init
6. 创建一个 CSS 文件，请使用 @tailwind 指令注入 Tailwind 的基础 (base)，组件 (components) 和功能 (utilities) 样式
```
/* ./your-css-folder/styles.css */
@tailwind base;
@tailwind components;
@tailwind utilities;
```
7. 现在打开package.json文件,写入以下脚本
```
"scripts": {
  "build-css": "postcss src/css/base.css -o src/dist/css/styles.css"
}
```
8. 构建最终CSS文件
```
npm run build-css
```

## 备注
1. trunk资料:https://trunkrs.dev/
2. yew官网地址: https://yew.rs/docs/tutorial

## 其他
1. reqwest在wasm下，不支持Proxy,Blocking模块.
2. reqwest网络请求跨域问题。
  * 简单请求设置响应头:Access-Control-Allow-Origin为“*”即可。
  * 非简单请求，CORS 机制跨域会首先进行 preflight（一个 OPTIONS 请求）， 该请求成功后才会发送真正的请求。
  * reqwest json格式发送的头部为小写:content-type, 需要在响应头Access-Control-Request-Headers中包含content-type
  * trunk代理模式
    - 在trunk配置文件Trunk.toml中启用proxy模块.
    - 设置backend="http://www.xxx.com/demo/test". 此地址为真实的地址
    - 设置rewrite="/api/".
    - reqwest请求的url设置为:http://127.0.0.1/api/demo/test
    - 请求时，http://127.0.0.1/api/会代理为http://www.xxx.com/demo/test
  * nginx代理



# wasm-pack编译管理

## 工具安装:
1. cargo install wasm-pack
2. cargo install cargo-make
3. rustup target add wasm32-unknown-unknown

## 修改的地方
1. main.rs 文件修改为lib.rs
2. 在Cargo.toml里新增
  ```
  [lib]
  crate-type = ["cdylib", "rlib"]

  #新增依赖wasm-bindgen
  [dependencies]
  wasm-bindgen = "^0.2"
  ```
3. 修改lib.rs
  ```
  use wasm_bindgen::prelude::*;
  #[wasm_bindgen(start)]
  pub fn main(){
      wasm_logger::init(wasm_logger::Config::default());
      yew::start_app::<App>();
      Ok(())
  }
  ```
4. index.html新增
  ```
  <script type="module">
    import init from "/pkg/wasm.js";
    init();
  </script>
  ```
5. 在admin目录下，新增Makefile.toml
  ```
  [tasks.build]
  args = ["build", "--dev", "--target", "web", "--out-name", "wasm", "--out-dir", "./static/dist/pkg"]
  command = "wasm-pack"
  watch = {ignore_pattern = "static/public/*"}

  # [tasks.serve]
  # command = "simple-http-server"
  # args = ["-i", "./static/public", "-p", "8080", "--nocache", "--try-file", "./static/public/index.html"]
  ```
## 启动服务
1. cargo make build

## 服务器相关
1. nginx单页面配置
  ```
  location / {
    index  index.html index.htm index.php;
    try_files $uri $uri/ /index.html;
  }
  ```
