use bevy::prelude::*;
use hello_bevy;

fn main() {

    /*
    Book oficial define em funções aninhada, que tecnicamente fazem grande
    sentindo, mas pode dificultar. 

    Seu sistema é semelhante a API GDNative da godot, e semelhante como criei
    o sistema de renderização em pilha dentro da godot para facilitar a defi-
    nição de quais compoentnes renderizar ou não de uma maneira mais organi-
    da.

    ```
    Original: 
        App::new()
            .add_system(hello_bevy::hello_bevy) <- Passo o ponteiro da função
                <- Retorna Self Mutável
            .run() <- Executa as as subrotinas previamente adicionadas.
            
    ```

    Complementação: Esqueci que ele opera em paralelo, isso quer dizer que
    não possuí o sistema simples de pilha que eu imaginei, ao adicionar na
    ordem.

    */

    // let mut app = App::new();

    // app.add_system(hello_bevy::first_test::hello_bevy);

    // app.run();


    let mut app = App::new();

    // De acordo com a doc, tudo é plugin em cima de ECS.
    // Possos estar enganado, mas isso me lembra o sistema de adição da vulkan,
    // onde opera manualmente adicionando, os componentes do sistema.
    app.add_plugins(DefaultPlugins);
    app.add_plugin(hello_bevy::my_first_plugin::HelloPlugin);
    
    // Ainda não compreendi bem o funcionamento deste abaixo.
    /*
    Pelo nome da impressão que ela será inicializada nas estapa inicial do sis-
    tema, de acordo com a lista de processos, mas como ele paralelisa com o add
    system, qeu suspostamente teria sua adição pós startup, imagino que venha a
    se demonstrar que o add possa, que nem os comentários dele, possa ser alte-
    rado realmente.
    */
    // app.add_startup_system(hello_bevy::components_test::add_people);

    // Registra as funções do sistema
    // app.add_system(hello_bevy::components_test::greet_people);
    // app.add_system(hello_bevy::first_test::hello_bevy);
    app.run()



}
