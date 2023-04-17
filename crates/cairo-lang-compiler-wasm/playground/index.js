import init, {build_db} from '../pkg';
init()
    .then(async () => {
        const p = await build_db("te");
        console.log(p);
    });