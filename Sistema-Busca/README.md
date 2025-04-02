# 🚀 Como executar o sistema

### 1️⃣ Pré-requisitos

Certifique-se de ter o Rust instalado. Para instalar, siga as instruções no site oficial: Rust - Install.
Siga os passo no manual do rust nesse site:
https://www.rust-lang.org/pt-BR/tools/install

Verifique a instalação:

- **rustc --version**
- **cargo --version**

### 2️⃣ Compilar o projeto

- **cargo build --release**

### 3️⃣ Executar o sistem

- **cargo run**

### ▶️ Como usar o sistema

Listar todas as categorias e produtos disponíveis:

- **catalogo.listar_todos();**

Buscar produtos por categoria:

- **catalogo.buscar_por_categoria("eletronicos");**

Listar buscas frequentes:

- **catalogo.listar_buscas_frequentes();**

Exibir recomendações de produtos:

- **catalogo.exibir_recomendacoes("Smartphone");**



