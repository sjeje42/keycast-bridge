import Cairo from 'cairo';
const TAU = Math.PI * 2;
function color(c, hex, alpha = 1) { c.setSourceRGBA(parseInt(hex.slice(1,3),16)/255, parseInt(hex.slice(3,5),16)/255, parseInt(hex.slice(5,7),16)/255, alpha); }
function circle(c,x,y,r) { c.newSubPath(); c.arc(x,y,r,0,TAU); }
function ring(c,x,y,r,width,hex,alpha,neon) {
    if (neon) for (let n=3;n>=1;n--) { color(c,hex,alpha*0.09); c.setLineWidth(width+n*5); circle(c,x,y,r); c.stroke(); }
    color(c,hex,alpha); c.setLineWidth(width); circle(c,x,y,r); c.stroke();
}
export function draw(c,w,h,scene,a,mode,t) {
    const s=a.presentation, r=a.ring, [x,y]=scene.pointer;
    c.setOperator(Cairo.Operator.CLEAR); c.paint(); c.setOperator(Cairo.Operator.OVER);
    if (mode==='whiteboard') { c.setSourceRGB(1,1,1); c.paint(); }
    if (s.spotlight && mode==='normal') {
        c.setSourceRGBA(0,0,0,s.dimming/100); c.rectangle(0,0,w,h);
        if(s.spotlight_shape==='square') c.rectangle(x-s.spotlight_radius,y-s.spotlight_radius,s.spotlight_radius*2,s.spotlight_radius*2);
        else circle(c,x,y,s.spotlight_radius);
        c.setFillRule(Cairo.FillRule.EVEN_ODD); c.fill(); c.setFillRule(Cairo.FillRule.WINDING);
    }
    c.setLineCap(Cairo.LineCap.ROUND); c.setLineJoin(Cairo.LineJoin.ROUND);
    if (s.trail && mode==='normal') {
        const dots = ['particle','star'].includes(s.trail_style);
        if (s.trail_style==='dash') c.setDash([12,10],0);
        if (s.trail_style==='dot') c.setDash([0.1,10],0);
        for(let i=1;i<scene.trail.length;i++) {
            const p=scene.trail[i], prev=scene.trail[i-1], alpha=Math.max(0,1-(t-p[2])/0.65);
            color(c,r.color,alpha*0.85); c.setLineWidth(r.thickness);
            if(dots) {
                if(i%3!==0) continue;
                if(s.trail_style==='star') {
                    for(let k=0;k<10;k++) { const angle=k*Math.PI/5-Math.PI/2, size=(k%2?3:7)*alpha; const px=p[0]+Math.cos(angle)*size,py=p[1]+Math.sin(angle)*size; if(k===0)c.moveTo(px,py);else c.lineTo(px,py); } c.closePath(); c.fill();
                } else { circle(c,p[0],p[1],Math.max(1,4*alpha));c.fill(); }
            } else {
                if(s.trail_style==='glow') { for(let j=3;j>=1;j--) {color(c,r.color,alpha*0.07);c.setLineWidth(r.thickness+j*4);c.moveTo(...prev.slice(0,2));c.lineTo(...p.slice(0,2));c.stroke();} color(c,r.color,alpha);c.setLineWidth(r.thickness); }
                c.moveTo(...prev.slice(0,2));c.lineTo(...p.slice(0,2));c.stroke();
            }
        } c.setDash([],0);
    }
    if(s.ring && mode==='normal') {
        ring(c,x,y,r.radius,r.thickness,r.color,r.pulse?0.75+0.25*Math.sin(t*4):1,r.neon);
        for(const p of scene.clicks) {const progress=(t-p[2])/0.5;ring(c,p[0],p[1],r.radius*(0.7+progress*0.7),r.thickness,r.color,1-progress,r.neon);}
    }
    if(scene.until>t && mode==='normal') ring(c,x,y,70+(scene.until-t)*90,5,r.color,Math.min(1,(scene.until-t)*3),true);
    for(const stroke of scene.strokes) {
        color(c,stroke.color); c.setLineWidth(stroke.width);
        if(stroke.points.length===1) {circle(c,...stroke.points[0],stroke.width/2);c.fill();}
        else {c.moveTo(...stroke.points[0]); for(const p of stroke.points.slice(1))c.lineTo(...p);c.stroke();}
    }
    if(mode!=='normal') {
        c.setSourceRGBA(0.08,0.09,0.12,0.85);c.rectangle(12,12,Math.min(610,w-24),36);c.fill();
        c.setSourceRGB(1,1,1);c.selectFontFace('Sans',Cairo.FontSlant.NORMAL,Cairo.FontWeight.NORMAL);c.setFontSize(15);c.moveTo(22,35);
        c.showText(mode==='zoom'?'Zoom · Scroll / + / − · Esc':'Draw · Ctrl+Z: undo · Backspace: clear · Esc');
    }
}
