import {Logo} from './logo'
import init, {run_command} from '../../rust/pkg'


export function App() {
    init();

    function click() {
        const cmd = prompt("Command:") || "echo foo"
        run_command(cmd);
    }

    return (
        <>
            <Logo/>
            <p onClick={click}>Hello Vite + Preact!</p>
        </>
    )
}
