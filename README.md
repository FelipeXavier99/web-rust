# web-rust - (pra clonar projetos iniciais)
Setup com minijinja e actixweb Bancos de dados Postgre com SQLX

```cmd
git clone
watch
```
## Adicionar uma rota

```diff
//abra o arquivo controller
//adicione um novo handler para sua nova rota
+ pub mod fornecedor{
```

```diff
+#[get("/json")]
+async fn get_vendas_json(pool: web::Data<AppState>, 
+    // query: web::Query<QueryParams>,
+    ) 
+    -> impl Responder {
+    
+    let pool = &pool.database.conn;
+    let result = RepositoryVendas::get_sales_by_period(&pool, 
+        // &query.start_date, 
+        // &query.end_date,
+    ).await;
+
+    match result {
+        Ok(sales) => HttpResponse::Ok().json(sales),
+       Err(_) => HttpResponse::InternalServerError().finish(),
+   }
+}
+}
```
## Adicione uma nova view

```html
// adicione um novo template html na pasta templates
{% for fornecedor in fornecedores %}
    <h1>{{ fornecedor.nome }}</h1>
    <h3>{{ fornecedor.endereco }}</h3>
{% endfor %}
```

```diff
// abra o arquivo view.rs
// adicione a chamada ao renderizador
+pub mod vendas {
+    use actix_web::Responder;
+    use minijinja::context;
+    // use serde_json::json;
+
+    use crate::models::ResumoVendas;
+
+    pub fn view_vendas(vendas: Vec<ResumoVendas>) -> impl Responder {
+        // let vendas = json!([{"cliente_nome": "ACME Corp", "total_vendido": 10000}]);
+
+        crate::render::render_minijinja(
+            "view_vendas.html", 
+            context!(vendas),
+        )
+    }
+}
```

## Chamar a view do controller
```diff
//abra o arquivo controller
//adicione um novo handler para sua nova view
+ pub mod view_fornecedor{
+#[get("/")]
+async fn get_vendas(pool: web::Data<AppState>,
+    // query: web::Query<QueryParams>,
+    ) 
+    -> impl Responder { 
+        let pool = &pool.database.conn;
+        let vendas = RepositoryVendas::get_sales_by_period(&pool, 
+            // &query.start_date, 
+            // &query.end_date,
+        ).await;
+        crate::view::vendas::view_vendas(vendas.unwrap())
+    }
```

```dif
//abra o arquivo routes
//registre o servico
+             .service(get_vendas)
```
