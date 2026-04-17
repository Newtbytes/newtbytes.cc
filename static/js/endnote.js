import {is_mobile, debounce} from "./utils.js"; 

var Endnote = {
    id: {
        wrapper: "endnote-container",
        get content() { return this.wrapper + '-content' }
    },

    width: () => window.innerWidth - 300 < 100 ? window.innerWidth - 60 : 300,
    height: 150,

    exists: false,

    setup: () => {
        document.querySelectorAll('sup.footnote-reference').forEach(
            node => {
                node.addEventListener('mouseover', is_mobile() ? Endnote.create : debounce(Endnote.create));
                node.addEventListener('mouseout', Endnote.remove);
                node.addEventListener('click', e => e.preventDefault());
            }
        );
    },

    create: (event) => {
        if (Endnote.exists) return;

        let padding = 16;
        let transparent_margin = 20;
        let width = Endnote.width();
        let height = Endnote.height;
        let x = event.x;
        let y = event.y;

        let wrapper = document.createElement('div');
        wrapper.id = Endnote.id.wrapper;
        wrapper.style.position = 'fixed';
        wrapper.style.left = `${x}px`;
        wrapper.style.top =`${y}px`;
        wrapper.innerHTML = `<div id="${Endnote.id.content}"></div>`;

        let dlg = wrapper.querySelector('#' + Endnote.id.content);
        dlg.style.marginTop = transparent_margin + 'px';
        dlg.style.marginBottom = transparent_margin + 'px';
        dlg.style.border = `1px solid lightgray`;
        dlg.style.boxShadow = 'rgba(100, 100, 111, 0.5) 0px 7px 14px 0px';
        dlg.style.borderRadius = '5px';
        dlg.style.background = 'white';
        dlg.style.color = '#191919';
        dlg.style.padding = padding + 'px';
        dlg.style.overflowY = 'auto';
        dlg.style.width = width + 'px';
        dlg.style.height = height + 'px';
        
        // add footnote contents
        document.querySelectorAll('.footnote-definition').forEach(value => {
            if (event.target.innerHTML == value.querySelector('.footnote-definition-label').innerHTML) {
                value.childNodes.forEach(child => {
                    dlg.append(child.cloneNode(true));
                })
            }
        });
        
        document.querySelector('body').appendChild(wrapper);

        Endnote.exists = true;
    },

    remove: (event) => {
        if (!Endnote.exists) return;

        let dlg = document.querySelector('#' + Endnote.id.wrapper);
        if (!dlg) return;

        document.querySelectorAll('#' + Endnote.id.wrapper).forEach(div => {
            if (dlg === event?.relatedTarget) return;
            div.remove();
        });
    }
};

Endnote.setup();