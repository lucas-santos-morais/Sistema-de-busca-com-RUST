use std::collections::{HashMap, HashSet};

// Estrutura do catálogo com suporte a pré-processamento e cache
struct Catalogo {
    tabela_hash: HashMap<String, Vec<String>>,
    cache: HashMap<String, Vec<String>>,
    buscas_frequentes: HashSet<String>, // Para rastrear buscas populares
}

impl Catalogo {
    // Função para criar um novo catálogo
    fn novo() -> Self {
        let mut tabela_hash = HashMap::new();
        tabela_hash.insert(String::from("Eletrônicos"), vec![String::from("Smartphone"), String::from("Laptop")]);
        tabela_hash.insert(String::from("Vestuário"), vec![String::from("Camiseta"), String::from("Calça")]);
        tabela_hash.insert(String::from("Decoração"), vec![String::from("Vaso"), String::from("Quadro")]);
        tabela_hash.insert(String::from("Alimentos"), vec![String::from("Chocolate"), String::from("Café")]);

        Catalogo {
            tabela_hash,
            cache: HashMap::new(),
            buscas_frequentes: HashSet::new(),
        }
    }

    // Função para remover stop words
    fn remover_stop_words(&self, texto: &str, stop_words: &[&str]) -> String {
        texto.split_whitespace()
            .filter(|palavra| !stop_words.contains(palavra))
            .collect::<Vec<&str>>()
            .join(" ")
    }

    // Função para simular stemming (simplificada para o exemplo)
    fn aplicar_stemming(&self, texto: &str) -> String {
        texto.to_lowercase() // Normalizamos para letras minúsculas
            .replace("s", "") // Exemplo: remover plural
            .replace("ção", "co") // Exemplo: simplificação de palavras
    }

    // Função para corrigir erros de ortografia (simulação)
    fn corrigir_ortografia(&self, texto: &str) -> String {
        if texto.to_lowercase() == "eletronicos" {
            "Eletrônicos".to_string()
        } else {
            texto.to_string()
        }
    }

    // Função para buscar produtos por categoria, com suporte a pré-processamento e cache
    fn buscar_por_categoria(&mut self, termo_busca: &str) {
        let stop_words = ["de", "e", "o", "a", "os", "as"];
        let termo_corrigido = self.corrigir_ortografia(termo_busca);
        let termo_sem_stop_words = self.remover_stop_words(&termo_corrigido, &stop_words);
        let termo_normalizado = self.aplicar_stemming(&termo_sem_stop_words);

        if let Some(produtos) = self.cache.get(&termo_normalizado) {
            println!("(Cache) Produtos encontrados na categoria '{}': {}", termo_busca, produtos.join(", "));
            return;
        }

        if let Some(produtos) = self.tabela_hash.get(&termo_normalizado) {
            println!("Produtos encontrados na categoria '{}': {}", termo_busca, produtos.join(", "));
            self.cache.insert(termo_normalizado.to_string(), produtos.clone());
            self.registrar_busca(&termo_normalizado);
        } else {
            println!("Categoria '{}' não encontrada.", termo_busca);
        }
    }

    // Função para registrar buscas frequentes
    fn registrar_busca(&mut self, categoria: &str) {
        self.buscas_frequentes.insert(categoria.to_string());
    }

    // Função para listar buscas frequentes
    fn listar_buscas_frequentes(&self) {
        if self.buscas_frequentes.is_empty() {
            println!("Nenhuma busca frequente registrada.");
        } else {
            println!("Buscas frequentes: {:?}", self.buscas_frequentes);
        }
    }

    // Função para listar todas as categorias e produtos
    fn listar_todos(&self) {
        for (categoria, produtos) in &self.tabela_hash {
            println!("Categoria: {} - Produtos: {}", categoria, produtos.join(", "));
        }
    }
}

// Testes unitários
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criar_catalogo() {
        let catalogo = Catalogo::novo();
        assert_eq!(catalogo.tabela_hash.len(), 4); // Deve haver 4 categorias
        assert!(catalogo.tabela_hash.contains_key("Eletrônicos"));
        assert!(catalogo.tabela_hash.contains_key("Vestuário"));
        assert!(catalogo.tabela_hash.contains_key("Decoração"));
        assert!(catalogo.tabela_hash.contains_key("Alimentos"));
    }

    #[test]
    fn test_remover_stop_words() {
        let catalogo = Catalogo::novo();
        let texto = "os melhores produtos de eletronicos";
        let stop_words = ["de", "e", "o", "os", "as"];
        let resultado = catalogo.remover_stop_words(texto, &stop_words);
        assert_eq!(resultado, "melhores produtos eletronicos");
    }

    #[test]
    fn test_aplicar_stemming() {
        let catalogo = Catalogo::novo();
        let texto = "eletronicos";
        let resultado = catalogo.aplicar_stemming(texto);
        assert_eq!(resultado, "eletronico");
    }

    #[test]
    fn test_corrigir_ortografia() {
        let catalogo = Catalogo::novo();
        let texto = "eletronicos";
        let resultado = catalogo.corrigir_ortografia(texto);
        assert_eq!(resultado, "Eletrônicos");
    }

    #[test]
    fn test_buscar_por_categoria() {
        let mut catalogo = Catalogo::novo();

        // Realiza a busca com um termo que passa pelo pré-processamento
        catalogo.buscar_por_categoria("eletronicos");
        
        // Replicar o processo de pré-processamento utilizado na função buscar_por_categoria
        let stop_words = ["de", "e", "o", "a", "os", "as"];
        let termo_corrigido = catalogo.corrigir_ortografia("eletronicos");
        let termo_sem_stop_words = catalogo.remover_stop_words(&termo_corrigido, &stop_words);
        let termo_normalizado = catalogo.aplicar_stemming(&termo_sem_stop_words);

        // Agora verifica se o termo pré-processado está no cache
        assert!(catalogo.cache.contains_key(&termo_normalizado));
    }

    #[test]
    fn test_registrar_busca_frequente() {
        let mut catalogo = Catalogo::novo();
        catalogo.registrar_busca("Eletrônicos");
        assert!(catalogo.buscas_frequentes.contains("Eletrônicos"));
    }
}