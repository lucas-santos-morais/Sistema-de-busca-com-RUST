## 🖥️Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

### Descrição do projeto

O projeto tem por finalidade construir um sistema de busca rápido, eficiênte e com segurança. A escolha da linguagem RUST atende aos critérios do projeto, técnicas com uso de grafos, tabela hash, stop world auxiliaram a contrubiur com recomendações aos clientes, dados ficam salvo na memória e permite recomendar produtos da mesma categoria.

### 🛠️ Tecnologias utilizadas

- **Rust** - Linguagem de programação utilizada no projeto
- **HashMap** e HashSet - Estruturas de dados para indexação eficiente dos produtos
- **Grafo** - Representação para recomendações de produtos
- **Pré-processamento de texto** - Correção de ortografia, remoção de stop words e aplicação de stemming

### 📝 Arquitetura e algoritmos

- **Tabelas hash**: utilizadas para armazenar categorias e produtos, permitindo buscas rápidas.
- **Grafo**: estrutura para relações entre produtos, oferecendo recomendações.
- **Pré-processamento de texto**: correção de erros de ortografia e remoção de palavras irrelevantes para melhorar buscas.

### 📊 Desempenho e escalabilidade

O sistema é projetado para ser escalável, garantindo buscas eficientes mesmo com um catálogo extenso. A inclusão de cache reduz o tempo de resposta em pesquisas repetidas.

### 📂PASTAS

Neste repositório tem as seguintes pastas:
- Sistema-Busca: Está o código completo
- testes: Testes unitários de partes do código completo

### 🤝 Como contribuir

Quer melhorar esse projeto? Sinta-se à vontade para contribuir!

**Sugestões e Feedback**<div>
Se tiver ideias para melhorias ou encontrar algum problema, abra uma Issue no repositório e descreva sua sugestão ou bug.

**Documentação**<div>
Se quiser melhorar o **README.md**, adicionar mais detalhes técnicos ou contribuir com exemplos de uso, é só enviar um Pull Request!


