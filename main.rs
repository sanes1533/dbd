/*O seu código atual causará um erro de compilação porque o Rust exige que todos os caminhos possíveis de código retornem um valor. No seu exemplo, o compilador não sabe o que retornar se a condição if teste1 for falsa.
A forma correta e mais idiomática de resolver isso em Rust é garantir um retorno padrão (mecanismo de fallback). Como o Rust é uma linguagem baseada em expressões, você pode omitir a palavra-chave return e colocar o valor diretamente.

Código Corrigido*/

fn dbd() -> &'static str {
    let deadbydaylight = true;
    let jogo = "jogo funcionando";
    
    if deadbydaylight {
        jogo
    } else {
        "jogo nao funcionando" 
    }
}
