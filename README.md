# API de Usuários em Rust

Este projeto implementa uma API RESTful simples para gerenciamento de usuários, desenvolvida com foco em alta performance e multi-threading utilizando a linguagem Rust.

## Visão Geral

O projeto consiste em uma API que permite as operações básicas de CRUD (Create, Read, Update, Delete) para entidades de usuário, utilizando PostgreSQL como banco de dados e com uma arquitetura organizada por módulos.

### Tecnologias Utilizadas

- **Linguagem**: Rust
- **Framework Web**: Actix Web - Um framework web rápido e pragmático
- **ORM**: Diesel - ORM e Query Builder para Rust
- **Banco de Dados**: PostgreSQL
- **Serialização/Deserialização**: Serde

## Estrutura do Projeto

```
src/
├── db.rs            # Configuração de conexão com o banco de dados
├── main.rs          # Ponto de entrada da aplicação e definição de rotas
├── models/          # Definição dos modelos de dados
│   ├── mod.rs
│   └── user.rs      # Modelo de usuário
├── repository/      # Camada de acesso a dados
│   ├── mod.rs
│   └── user_repository.rs  # Operações de CRUD para usuários
└── schema.rs        # Schema do banco de dados gerado pelo Diesel
```

## Modelos de Dados

### User

```rust
// Modelo para novos usuários (usado na criação)
pub struct NewUser {
    pub name: String,
    pub email: String,
}

// Modelo para usuários existentes (retornado nas consultas)
pub struct Users {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// Modelo para atualização de usuários
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}
```

## Endpoints da API

A API expõe os seguintes endpoints:

| Método | Endpoint      | Descrição                   | Corpo da Requisição | Resposta de Sucesso |
|--------|---------------|-----------------------------|--------------------|---------------------|
| POST   | /users        | Cria um novo usuário        | `NewUser`          | `Users`             |
| GET    | /users/{id}   | Obtém um usuário pelo ID    | -                  | `Users`             |
| PUT    | /users/{id}   | Atualiza um usuário pelo ID | `UpdateUser`       | `Users`             |
| DELETE | /users/{id}   | Remove um usuário pelo ID   | -                  | "User deleted successfully" |

## Configuração do Banco de Dados

O projeto utiliza variáveis de ambiente para configurar a conexão com o banco de dados PostgreSQL. É necessário definir a variável `DATABASE_URL` no formato:

```
DATABASE_URL=postgres://username:password@localhost/database_name
```

O projeto utiliza o pacote `dotenv` para carregar essa variável de um arquivo `.env` na raiz do projeto.

## Como Executar

### Pré-requisitos

1. Rust e Cargo instalados
2. PostgreSQL instalado e configurado
3. Diesel CLI instalado (`cargo install diesel_cli --no-default-features --features postgres`)

### Passos

1. Clone o repositório
2. Crie um arquivo `.env` na raiz do projeto com a variável `DATABASE_URL`
3. Configure o banco de dados:
   ```bash
   diesel setup
   diesel migration run
   ```
4. Execute o projeto:
   ```bash
   cargo run
   ```
5. A API estará disponível em `http://127.0.0.1:8080`

## Características de Performance

O projeto foi desenvolvido com foco em performance, aproveitando os seguintes recursos:

1. **Rust**: Linguagem de programação com desempenho comparável ao C/C++ e segurança de memória garantida em tempo de compilação.

2. **Actix Web**: Framework web assíncrono de alto desempenho que implementa o modelo Actor, permitindo processamento concorrente eficiente.

3. **Workers**: Configuração de múltiplos workers para aproveitar processamento multi-thread:
   ```rust
   .workers(2)
   ```

4. **Diesel**: ORM que gera SQL otimizado e eficiente, com verificações em tempo de compilação.

5. **Connection Pool**: Uso implícito de pool de conexões para otimizar o acesso ao banco de dados.

## Possíveis Melhorias

1. Implementar autenticação e autorização
2. Adicionar validação de dados mais robusta
3. Implementar paginação nas consultas
4. Adicionar logs estruturados
5. Implementar cache para operações frequentes
6. Configurar testes automatizados
7. Documentação da API com Swagger/OpenAPI
8. Implementar gestão de migrations do banco de dados

## Considerações de Segurança

- As senhas dos usuários não estão sendo armazenadas nesta implementação básica
- Para um ambiente de produção, seria necessário implementar hashing de senhas, tokens JWT, HTTPS, etc.
- Validação de entrada precisa ser aprimorada para prevenir ataques de injeção

---

Este projeto serve como um ponto de partida para o estudo de desenvolvimento de APIs em Rust, demonstrando como construir aplicações web de alta performance com um foco em concorrência e eficiência.
