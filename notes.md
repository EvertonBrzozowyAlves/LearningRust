# Resumo
**Rust** é uma linguagem compilada, mais próxima a **C** e **C++**.  
Página oficial da linguagem pode ser encontrada [aqui](https://www.rust-lang.org/).  

Tem um sistema de gerenciamento de memória bastante seguro.  
Quando a memória não é limpa automaticamente, lhe é informado que essa memória deve ser liberada.  

Rust tem um compilador bastante elogiado, com mensagens de erro mais intuitivas.  
Tem também um ecossistema bastante interessante.  

Rust já é utilizado em boa parte do Firefox e vem sido recomendado para substituir linguagens de baixo nível em vários sistemas, como partes do Windows.  

## Instalação
Link da pagina de instalação [aqui](https://www.rust-lang.org/tools/install).  
> Pode ser necessário baixar e instalar as ferramentas de compilação para C++ do Visual Studio (Visual Studio C++ Build tools).  

Por padrão, é criada uma pasta da instalação da linguagem na pasta home do seu usuário.  
```
C:\Users\<user>\.cargo
```

Utilizamos o termo **rustup** para gerenciar projetos Rust.  

Para atualizar a instalação.
```powershell
rust udpate
```

Para compilar o código, utilizamos o **rustc**:
```powershell
rustc file.rs
```
> Será gerado um arquivo executável 

## Tipos
Link dos tipos primitivos pode ser encontrado [aqui](https://doc.rust-lang.org/std/index.html#primitives).  

|type                   | description
|:----------------------|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
|never *Experimental*	| The ! type, also called “never”.
|array	                | A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
|bool	                | The boolean type.
|char	                | A character type.
|f32	                | A 32-bit floating point type (specifically, the “binary32” type defined in IEEE 754-2008).
|f64	                | A 64-bit floating point type (specifically, the “binary64” type defined in IEEE 754-2008).
|fn	                    | Function pointers, like fn(usize) -> bool.
|i8	                    | The 8-bit signed integer type.
|i16	                | The 16-bit signed integer type.
|i32	                | The 32-bit signed integer type.
|i64	                | The 64-bit signed integer type.
|i128	                | The 128-bit signed integer type.
|isize	                | The pointer-sized signed integer type.
|pointer	            | Raw, unsafe pointers, *const T, and *mut T.
|reference	            | References, both shared and mutable.
|slice	                | A dynamically-sized view into a contiguous sequence, [T]. Contiguous here means that elements are laid out so that every element is the same distance from its neighbors.
|str	                | String slices.
|tuple	                | A finite heterogeneous sequence, (T, U, ..).
|u8	                    | The 8-bit unsigned integer type.
|u16	                | The 16-bit unsigned integer type.
|u32	                | The 32-bit unsigned integer type.
|u64	                | The 64-bit unsigned integer type.
|u128	                | The 128-bit unsigned integer type.
|unit	                | The () type, also called “unit”.
|usize	                | The pointer-sized unsigned integer type.

