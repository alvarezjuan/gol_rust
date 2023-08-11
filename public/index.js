const API_URL_BASE = 'http://localhost:8080';

const get_x = () => {
    return Number(document.getElementById('x').value);
};
const set_x = (vx) => {
    return document.getElementById('x').value = vx;
};
const get_y = () => {
    return Number(document.getElementById('y').value);
};
const set_y = (vy) => {
    return document.getElementById('y').value = vy;
};
const get_w = () => {
    return Number(document.getElementById('w').value);
};
const set_w = (vw) => {
    return document.getElementById('w').value = vw;
};
const get_h = () => {
    return Number(document.getElementById('h').value);
};
const set_h = (vh) => {
    return document.getElementById('h').value = vh;
};
const get_d = () => {
    return Number(document.getElementById('d').value);
};
const set_d = (vd) => {
    return document.getElementById('d').value = vd;
};
const get_image_api = () => {
    let lx = Math.trunc(get_x() - get_w() / 2);
    let ly = Math.trunc(get_y() - get_h() / 2);
    document.getElementById('i').src = API_URL_BASE + '/getsvg?x=' + lx + '&y=' + ly + '&w=' + get_w() + '&h=' + get_h() + '&r=' + Math.random();
};
const game_refresh = () => {
    get_image_api();
    setTimeout(game_refresh, get_d());
};
const key_handler = (event) => {
    if (event.defaultPrevented) {
        return;
    };
    event.preventDefault();
    if (event.key === 'Home') {
        set_x(0);
        set_y(0);
        set_w(300);
        set_h(200);
        set_d(100);
    } else if (event.key === 'End') {
        set_x(0);
        set_y(0);
        set_w(1024);
        set_h(1024);
        set_d(1000);
    } else if (event.key === 'ArrowDown') {
        set_y(get_y() + 10);
    } else if (event.key === 'ArrowUp') {
        set_y(get_y() - 10);
    } else if (event.key === 'ArrowLeft') {
        set_x(get_x() - 10);
    } else if (event.key === 'ArrowRight') {
        set_x(get_x() + 10);
    } else if (event.key === 's') {
        set_y(get_y() + 10);
    } else if (event.key === 'S') {
        set_y(get_y() + 50);
    } else if (event.key === 'w') {
        set_y(get_y() - 10);
    } else if (event.key === 'W') {
        set_y(get_y() - 50);
    } else if (event.key === 'a') {
        set_x(get_x() - 10);
    } else if (event.key === 'A') {
        set_x(get_x() - 50);
    } else if (event.key === 'd') {
        set_x(get_x() + 10);
    } else if (event.key === 'D') {
        set_x(get_x() + 50);
    } else if (event.key === 'q') {
        set_h(get_h() + 10);
    } else if (event.key === 'Q') {
        set_h(get_h() - 10);
    } else if (event.key === 'e') {
        set_w(get_w() + 10);
    } else if (event.key === 'E') {
        set_w(get_w() - 10);
    } else {
        return;
    };
    get_image_api();
};
const main = () => {
    // Setup initial variable values
    set_x(0);
    set_y(0);
    set_w(300);
    set_h(200);
    set_d(1000);
    // Register event handlers
    window.addEventListener('load', () => game_refresh());
    window.addEventListener('keydown', (e) => key_handler(e));
};
main();