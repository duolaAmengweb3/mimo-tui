/* ============================================================
   MiMo-TUI — Premium Interactions
   ============================================================ */

document.addEventListener('DOMContentLoaded', () => {

    // ===== Scroll Progress Bar =====
    const scrollProgress = document.getElementById('scrollProgress');
    window.addEventListener('scroll', () => {
        const scrollTop = window.scrollY;
        const docHeight = document.documentElement.scrollHeight - window.innerHeight;
        const progress = docHeight > 0 ? (scrollTop / docHeight) * 100 : 0;
        scrollProgress.style.width = progress + '%';
    }, { passive: true });

    // ===== Navbar scroll effect =====
    const navbar = document.querySelector('.navbar');
    window.addEventListener('scroll', () => {
        navbar.classList.toggle('scrolled', window.scrollY > 50);
    }, { passive: true });

    // ===== Cursor Glow =====
    const cursorGlow = document.getElementById('cursorGlow');
    let mouseX = -600, mouseY = -600;

    document.addEventListener('mousemove', (e) => {
        mouseX = e.clientX;
        mouseY = e.clientY;
        cursorGlow.style.left = mouseX + 'px';
        cursorGlow.style.top  = mouseY + 'px';
    });

    // ===== Mobile menu toggle =====
    const menuToggle = document.querySelector('.menu-toggle');
    const navLinks = document.querySelector('.nav-links');

    menuToggle.addEventListener('click', () => {
        navLinks.classList.toggle('active');
    });

    navLinks.querySelectorAll('a').forEach(link => {
        link.addEventListener('click', () => {
            navLinks.classList.remove('active');
        });
    });

    // ===== Smooth scroll =====
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', (e) => {
            const target = document.querySelector(anchor.getAttribute('href'));
            if (target) {
                e.preventDefault();
                const offset = 80;
                const top = target.getBoundingClientRect().top + window.scrollY - offset;
                window.scrollTo({ top, behavior: 'smooth' });
            }
        });
    });

    // ===== Copy to clipboard =====
    document.querySelectorAll('.copy-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const text = btn.dataset.clipboardText;
            navigator.clipboard.writeText(text).then(() => {
                const icon = btn.querySelector('i');
                icon.classList.replace('fa-copy', 'fa-check');
                btn.style.background = 'var(--accent)';
                btn.style.borderColor = 'var(--accent)';
                btn.style.color = '#fff';
                setTimeout(() => {
                    icon.classList.replace('fa-check', 'fa-copy');
                    btn.style.background = '';
                    btn.style.borderColor = '';
                    btn.style.color = '';
                }, 1500);
            });
        });
    });

    // ===== Scroll reveal with stagger =====
    const revealObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('visible');
                revealObserver.unobserve(entry.target);
            }
        });
    }, {
        threshold: 0.12,
        rootMargin: '0px 0px -60px 0px'
    });

    const staggerGroups = [
        '.features-grid .feature-card',
        '.workflow-steps .step',
        '.installation-methods .method',
        '.about-stats .stat-card'
    ];

    staggerGroups.forEach(selector => {
        document.querySelectorAll(selector).forEach((el, i) => {
            el.classList.add('fade-in');
            el.style.transitionDelay = `${i * 0.1}s`;
            revealObserver.observe(el);
        });
    });

    // ===== Terminal typing effect =====
    const commandEl = document.querySelector('.terminal-line .command');
    const responseLines = document.querySelectorAll('.terminal-line.response');

    if (commandEl) {
        const fullText = commandEl.textContent;
        commandEl.textContent = '';

        // Hide responses initially
        responseLines.forEach(line => line.style.opacity = '0');

        let started = false;

        function typeCommand(cb) {
            let i = 0;
            function next() {
                if (i < fullText.length) {
                    commandEl.textContent += fullText[i];
                    i++;
                    setTimeout(next, 50 + Math.random() * 40);
                } else if (cb) {
                    setTimeout(cb, 400);
                }
            }
            next();
        }

        function revealResponses() {
            responseLines.forEach((line, i) => {
                setTimeout(() => {
                    line.style.transition = 'opacity 0.5s ease, transform 0.5s ease';
                    line.style.transform = 'translateY(8px)';
                    line.style.opacity = '1';
                    requestAnimationFrame(() => {
                        line.style.transform = 'translateY(0)';
                    });
                }, i * 350);
            });
        }

        const terminalObserver = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting && !started) {
                    started = true;
                    setTimeout(() => typeCommand(revealResponses), 500);
                    terminalObserver.unobserve(entry.target);
                }
            });
        }, { threshold: 0.3 });

        const terminalWindow = document.querySelector('.terminal-window');
        if (terminalWindow) terminalObserver.observe(terminalWindow);
    }

    // ===== Terminal 3D tilt on mouse =====
    const terminalVisual = document.querySelector('.hero-visual');
    if (terminalVisual) {
        terminalVisual.addEventListener('mousemove', (e) => {
            const rect = terminalVisual.getBoundingClientRect();
            const x = (e.clientX - rect.left) / rect.width  - 0.5;
            const y = (e.clientY - rect.top)  / rect.height - 0.5;
            const terminal = terminalVisual.querySelector('.terminal-window');
            if (terminal) {
                terminal.style.animation = 'none';
                terminal.style.transform = `rotateY(${x * 8}deg) rotateX(${-y * 8}deg) translateY(-4px)`;
            }
        });

        terminalVisual.addEventListener('mouseleave', () => {
            const terminal = terminalVisual.querySelector('.terminal-window');
            if (terminal) {
                terminal.style.transform = '';
                terminal.style.animation = '';
            }
        });
    }

    // ===== Stat counter animation =====
    const statNumbers = document.querySelectorAll('.hero-stats .stat-number');
    const counterObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                animateCounter(entry.target);
                counterObserver.unobserve(entry.target);
            }
        });
    }, { threshold: 0.5 });

    statNumbers.forEach(el => counterObserver.observe(el));

    function animateCounter(el) {
        const text = el.textContent.trim();
        const match = text.match(/^(\d+)(\+?)$/);
        if (!match) return;

        const target = parseInt(match[1]);
        const suffix = match[2];
        const duration = 1200;
        const startTime = performance.now();

        function update(now) {
            const elapsed = now - startTime;
            const progress = Math.min(elapsed / duration, 1);
            // ease-out-expo
            const eased = 1 - Math.pow(1 - progress, 4);
            const current = Math.round(eased * target);
            el.textContent = current + suffix;
            if (progress < 1) requestAnimationFrame(update);
        }

        requestAnimationFrame(update);
    }

    // ===== Particles in hero =====
    const particlesContainer = document.getElementById('particles');
    if (particlesContainer) {
        function createParticle() {
            const particle = document.createElement('div');
            particle.classList.add('particle');
            const size = 2 + Math.random() * 3;
            const x = Math.random() * 100;
            const duration = 8 + Math.random() * 12;
            const hue = Math.random() > 0.5 ? '260' : '170'; // purple or teal

            particle.style.cssText = `
                width: ${size}px;
                height: ${size}px;
                left: ${x}%;
                bottom: -10px;
                background: hsl(${hue}, 80%, 70%);
                box-shadow: 0 0 ${size * 3}px hsl(${hue}, 80%, 60%);
                animation-duration: ${duration}s;
            `;

            particlesContainer.appendChild(particle);
            setTimeout(() => particle.remove(), duration * 1000);
        }

        // Spawn particles periodically
        setInterval(createParticle, 600);
        // Initial batch
        for (let i = 0; i < 8; i++) {
            setTimeout(createParticle, i * 200);
        }
    }

    // ===== Magnetic button effect =====
    document.querySelectorAll('.btn').forEach(btn => {
        btn.addEventListener('mousemove', (e) => {
            const rect = btn.getBoundingClientRect();
            const x = e.clientX - rect.left - rect.width / 2;
            const y = e.clientY - rect.top  - rect.height / 2;
            btn.style.transform = `translate(${x * 0.15}px, ${y * 0.15}px) translateY(-3px)`;
        });

        btn.addEventListener('mouseleave', () => {
            btn.style.transform = '';
        });
    });

    // ===== Nav active section highlight =====
    const sections = document.querySelectorAll('section[id]');
    const navAnchors = document.querySelectorAll('.nav-links a');

    const sectionObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const id = entry.target.getAttribute('id');
                navAnchors.forEach(a => {
                    a.style.color = a.getAttribute('href') === '#' + id
                        ? '#fff'
                        : '';
                });
            }
        });
    }, { threshold: 0.3, rootMargin: '-80px 0px -40% 0px' });

    sections.forEach(s => sectionObserver.observe(s));

});
