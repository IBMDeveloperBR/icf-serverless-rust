# Serverless Rust com IBM Cloud Functions

Este repositório contém um código esqueleto em Rust-lang pronto para ser implantado como uma serverless function na IBM Cloud Functions. É fornecido um Dockerfile baseado em Alpine Linux, com `musl` configurado para compilação multi-plataforma de código Rust.

## Pré-requisitos

- Uma conta registrada no DockerHub (ou outro registro de containers para salvar a imagem Docker que será criada);
- Uma conta registrada na IBM Cloud;
- O Docker deve estar instalado em seu computador;
- A IBM Cloud CLI deve estar instalada em seu computador;
- O plugin _cloud-functions_ deve estar instalado na IBM Cloud CLI.

Se você não possui o plugin _cloud-functions_, basta digitar o comando abaixo para instalá-lo:

`ibmcloud plugin install cloud-functions`

Para testar se tudo está instalado corretamente, você pode realizar uma invocação síncrona de echo, passando “hello” como argumento:

`ibmcloud fn action invoke /whisk.system/utils/echo -p message hello --result`

## Criando uma Serverless Function em Rust-Lang

Veja o passo-a-passo detalhado no [post do Medium sobre ICF](https://medium.com/@vnderlev/serverless-rust-com-apache-openwhisk-8a08b20cb94d)
