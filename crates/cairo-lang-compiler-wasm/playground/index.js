import init, {build_db, parser, build_db} from '../pkg';
init()
    .then(async () => {
        build_db("/fib.cairo", `// Calculates fib...
        fn fib(a: felt252, b: felt252, n: felt252) -> felt252 {
            match n {
                0 => a,
                _ => fib(b, a + b, n - 1),
            }
        }
        `)
    });