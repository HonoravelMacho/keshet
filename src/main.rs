use clap::Parser;
use rug::{Float, float::Constant, ops::Pow};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rayon::prelude::*;
use std::collections::HashSet;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use crossterm::event::{self, Event, KeyCode};

const PRECISAO_BITS: u32 = 128;

#[derive(Parser, Debug)]
#[command(name = "Keshet", author = "Tiago Rabelo (HonoravelMacho)", version = "12.2")]
struct Args {
    #[arg(short = 'g', long)] grau: Option<usize>,
    #[arg(short = 'a', long)] alcance: Option<i64>,
    #[arg(short = 'r', long)] resultados: Option<usize>,
    #[arg(short = 'n', long, num_args = 1.., allow_hyphen_values = true)] numeros: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
struct Termo {
    valor: Float,
    display: String,
    score: u32,
}

struct ConfigBusca {
    modo: u8, // 1: Rabeliana, 2: Grau 2, 3: Grau 3, 4: Somatórios
    alcance: i64,
    limite: usize,
    alvos: Vec<(Float, usize)>,
    n_somatorio: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(n_flags) = args.numeros {
        let alvos = n_flags.iter().map(|s| {
            let s_f = s.replace(",", ".");
            (Float::with_val(PRECISAO_BITS, Float::parse(&s_f).unwrap()), s_f.split('.').nth(1).map_or(0, |f| f.len()))
        }).collect();
        executar_busca(ConfigBusca { modo: args.grau.unwrap_or(2) as u8, alcance: args.alcance.unwrap_or(5), limite: args.resultados.unwrap_or(10), alvos, n_somatorio: 200 })?;
    } else {
        menu_principal()?;
    }
    Ok(())
}

fn menu_principal() -> anyhow::Result<()> {
    loop {
        println!("\n{}", "=================================================".cyan());
        println!("   {} - Universal Architect v12.2", "קֶשֶׁת (KESHET)".bold().yellow());
        println!("{}", "=================================================".cyan());
        println!(" 1. 🔍 Iniciar Busca Analítica (Identidade Rabeliana 3.0)");
        println!(" 2. 📖 Manual das Flags");
        println!(" 0. 🚪 Sair");
        print!("\nCapitão, sua ordem: ");
        io::stdout().flush()?;
        let mut opt = String::new();
        io::stdin().read_line(&mut opt)?;
        match opt.trim() {
            "1" => { menu_selecao_motor()?; },
            "2" => { mostrar_ajuda(); },
            "0" => { std::process::exit(0); },
            _ => (),
        }
    }
}

fn mostrar_ajuda() {
    println!("\n{}", "--- MANUAL KESHET v12.2 ---".yellow().bold());
    println!("Identidade Rabeliana 3.0: A * Base^(B*pi) + C/N^k + D = Alvo");
    println!("Filtro de Preguiça: Banidos automaticamente 0, 1, -1 e sqrt(quadrados perfeitos).");
}

fn menu_selecao_motor() -> anyhow::Result<()> {
    println!("\n--- Motores de Busca ---");
    println!(" 1. 🧬 Identidade Rabeliana 3.0 (Paramétrica Total)");
    println!(" 2. 📐 Polinômio de Grau 2");
    println!(" 3. 🧊 Polinômio de Grau 3");
    println!(" 4. 🌀 Somatórios de Elite (n variável)");
    println!(" 5. ⬅ Voltar");
    print!("\nEscolha: ");
    io::stdout().flush()?;
    let mut opt = String::new();
    io::stdin().read_line(&mut opt)?;
    let modo: u8 = match opt.trim() {
        "1" => 1, "2" => 2, "3" => 3, "4" => 4, _ => return Ok(()),
    };

    let mut n_somatorio = 200;
    if modo == 4 {
        n_somatorio = perguntar("Profundidade da série (n máximo): ").parse().unwrap_or(200);
    }

    let alcance: i64 = perguntar("Alcance Inteiro (A): ").parse().unwrap_or(5);
    let limite: usize = perguntar("Limite Resultados no Terminal: ").parse().unwrap_or(20);
    
    let mut alvos = Vec::new();
    let num_inputs = if modo == 1 || modo == 4 { 1 } else { modo as usize };
    for i in 1..=num_inputs {
        let mut input = perguntar(&format!("Alvo {}: ", i));
        if input.is_empty() { continue; }
        input = input.replace(",", ".");
        let prec = input.split('.').nth(1).map_or(0, |f| f.len());
        alvos.push((Float::with_val(PRECISAO_BITS, Float::parse(&input).expect("Erro")), prec));
    }
    executar_busca(ConfigBusca { modo, alcance, limite, alvos, n_somatorio })
}

fn gerar_universo_v12(alcance: i64) -> Vec<Termo> {
    let mut u = Vec::new();
    let bits = PRECISAO_BITS;
    let mut visto = HashSet::new();
    let mut bases = Vec::new();

    // QUINTETO DE OURO
    let elites = vec![
        (Float::with_val(bits, Constant::Pi), "π"),
        (Float::with_val(bits, 1.0).exp(), "e"),
        ((Float::with_val(bits, 5.0).sqrt() + 1.0) / 2.0, "Φ"),
        (Float::with_val(bits, Constant::Catalan), "G"),
        (Float::with_val(bits, Constant::Euler), "γ"),
    ];

    for (v, s) in elites {
        bases.push((v.clone(), s.to_string(), 1));
        let neg = Float::with_val(bits, &v * -1.0);
        bases.push((neg, format!("-{}", s), 2));
    }

    for n in 1..=alcance {
        let val_n = Float::with_val(bits, n);
        bases.push((val_n.clone(), n.to_string(), 10));
        bases.push((Float::with_val(bits, &val_n * -1.0), format!("-{}", n), 11));

        let root_f = (n as f64).sqrt();
        if root_f.fract() != 0.0 { 
            bases.push((Float::with_val(bits, n).sqrt(), format!("√{}", n), 20)); 
        }

        let ln_v = Float::with_val(bits, n as f64).ln();
        if Float::with_val(bits, &ln_v).abs() > 1e-15 && Float::with_val(bits, &ln_v - 1.0).abs() > 1e-15 {
            bases.push((ln_v, format!("ln({})", n), 25));
        }
    }

    let mut compostos = Vec::new();
    for i in 0..bases.len() {
        for j in 0..bases.len() {
            if i == j { continue; }
            let den = &bases[j].0;
            if den.clone().abs() < 1e-15 || Float::with_val(bits, den.clone() - 1.0).abs() < 1e-15 { continue; }
            
            let v_frac = Float::with_val(bits, &bases[i].0 / den);
            if v_frac.clone().abs() < 1e-15 || Float::with_val(bits, v_frac.clone() - 1.0).abs() < 1e-15 || Float::with_val(bits, v_frac.clone() + 1.0).abs() < 1e-15 { 
                continue; 
            }
            compostos.push((v_frac, format!("{}/{}", bases[i].1, bases[j].1), bases[i].2 + bases[j].2 + 5));
        }
    }

    for (v, s, sc) in bases.into_iter().chain(compostos.into_iter()) {
        let key = format!("{:.12}", v);
        if visto.insert(key) {
            u.push(Termo { valor: v, display: s, score: sc });
        }
    }

    u.sort_by(|a, b| a.score.cmp(&b.score));
    u
}

fn executar_busca(config: ConfigBusca) -> anyhow::Result<()> {
    let universo = gerar_universo_v12(config.alcance);
    let total_perm = match config.modo {
        1 => (universo.len() as u128).pow(6),
        2 => (universo.len() as u128).pow(3),
        3 => (universo.len() as u128).pow(4),
        4 => (universo.len() as u128).pow(3),
        _ => 0,
    };
    
    let encontrados = Arc::new(AtomicUsize::new(0));
    let pausado = Arc::new(AtomicBool::new(false));
    let p_clone = Arc::clone(&pausado);
    let banco = Arc::new(Mutex::new(HashSet::new()));

    println!("\n🚀 {} combinações na mira. FOGO!", format_compact(total_perm).magenta().bold());
    let pb = ProgressBar::new(universo.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.magenta/blue}] {pos}/{len} (ETA: {eta})").unwrap().progress_chars("█▉▊▋▌▍▎▏"));

    // Pre-calculo de Pi para o motor
    let pi = Float::with_val(PRECISAO_BITS, Constant::Pi);

    // Thread de Pausa
    std::thread::spawn(move || {
        loop {
            if event::poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if key.code == KeyCode::Char(' ') || key.code == KeyCode::Char('p') {
                        let estado = p_clone.fetch_xor(true, Ordering::SeqCst);
                        if !estado { println!(" {}", "⏸ PAUSADO".yellow().bold()); }
                        else { println!(" {}", "▶ RETOMANDO".green().bold()); }
                    }
                }
            }
        }
    });

    let inicio = Instant::now();
    let idx: Vec<usize> = (0..universo.len()).collect();

    idx.par_iter().for_each(|&i| {
        pb.inc(1);
        for &j in &idx {
            for &k in &idx {
                if encontrados.load(Ordering::Relaxed) >= config.limite { return; }
                while pausado.load(Ordering::Relaxed) { std::thread::sleep(std::time::Duration::from_millis(50)); }

                match config.modo {
                    1 => { // IDENTIDADE RABELIANA 3.0: A * Base^(Bpi) + C/N^k + D
                        let a = &universo[i];
                        let base = &universo[j];
                        let b_mult = &universo[k];
                        
                        // Veto Base Trivial
                        if Float::with_val(PRECISAO_BITS, &base.valor).abs() < 1e-15 || Float::with_val(PRECISAO_BITS, &base.valor - 1.0).abs() < 1e-15 { continue; }
                        
                        let expo_total = Float::with_val(PRECISAO_BITS, &b_mult.valor * &pi);
                        let p1 = Float::with_val(PRECISAO_BITS, base.valor.clone().pow(&expo_total));
                        let term1 = Float::with_val(PRECISAO_BITS, &a.valor * p1);

                        for &l in &idx { // C
                            let c_zeta = &universo[l];
                            for &m in &idx { // k
                                let k_zeta = &universo[m];
                                for d_idx in 1..=config.alcance.max(2) { // D (constante aditiva rápida)
                                    let n_val = (d_idx % config.alcance.max(2) as i64) + 2;
                                    let zeta_den = Float::with_val(PRECISAO_BITS, Float::with_val(PRECISAO_BITS, n_val).pow(&k_zeta.valor));
                                    let term2 = Float::with_val(PRECISAO_BITS, &c_zeta.valor / zeta_den);
                                    
                                    // Cálculo de soma segura em Rug
                                    let mut total = Float::with_val(PRECISAO_BITS, &term1 + &term2);
                                    total += 1.0; // Adicionando o +1 da identidade de Euler como teste base

                                    if validar_precisao(&total, &config.alvos) {
                                        registrar_rabeliana(a, base, b_mult, c_zeta, n_val as usize, k_zeta, inicio.elapsed().as_millis(), &encontrados, &banco, config.limite);
                                    }
                                }
                            }
                        }
                    },
                    2 => {
                        let c = [&universo[i], &universo[j], &universo[k]];
                        processar_polinomio(&c, &config, &encontrados, &banco, inicio);
                    },
                    3 => {
                        for &l in &idx {
                            let c = [&universo[i], &universo[j], &universo[k], &universo[l]];
                            processar_polinomio(&c, &config, &encontrados, &banco, inicio);
                        }
                    },
                    4 => { // SOMATÓRIOS
                        let a = &universo[i]; let b = &universo[j]; let k_exp = &universo[k];
                        if a.valor.clone().abs() < 1e-15 { continue; }
                        let mut acc = Float::with_val(PRECISAO_BITS, 0);
                        for n in 1..=config.n_somatorio {
                            let base = Float::with_val(PRECISAO_BITS, n) + &b.valor;
                            if base.clone().abs() > 1e-10 { 
                                acc += Float::with_val(PRECISAO_BITS, 1.0 / base.pow(&k_exp.valor)); 
                            }
                        }
                        let res = Float::with_val(PRECISAO_BITS, &a.valor * acc);
                        if validar_precisao(&res, &config.alvos) {
                            registrar_somatorio(a, b, k_exp, inicio.elapsed().as_millis(), &encontrados, &banco, config.limite);
                        }
                    },
                    _ => {}
                }
            }
        }
    });

    pb.finish_with_message("Missão Cumprida");
    Ok(())
}

fn validar_precisao(res: &Float, alvos: &[(Float, usize)]) -> bool {
    if alvos.is_empty() { return false; }
    let (alvo, prec) = &alvos[0];
    let margem = Float::with_val(PRECISAO_BITS, 10.0).pow(-(*prec as i32)) * 1.1;
    Float::with_val(PRECISAO_BITS, res - alvo).abs() < margem
}

fn registrar_rabeliana(a: &Termo, base: &Termo, b: &Termo, c: &Termo, n: usize, k: &Termo, ms: u128, enc: &Arc<AtomicUsize>, banco: &Arc<Mutex<HashSet<String>>>, limite: usize) {
    let sig = format!("RAB|{}|{}|{}|{}|{}|{}", a.display, base.display, b.display, c.display, n, k.display);
    if banco.lock().unwrap().insert(sig) && enc.fetch_add(1, Ordering::SeqCst) < limite {
        println!("\n🎯 IDENTIDADE RABELIANA 3.0: [{} ms]", ms);
        println!("  ({}) · {}^{}·π + ({})/({}^{}) + 1 = ALVO\n", a.display.magenta(), base.display.magenta(), b.display.magenta(), c.display.magenta(), n, k.display.magenta());
    }
}

fn registrar_somatorio(a: &Termo, b: &Termo, k: &Termo, ms: u128, enc: &Arc<AtomicUsize>, banco: &Arc<Mutex<HashSet<String>>>, limite: usize) {
    let sig = format!("SUM|{}|{}|{}", a.display, b.display, k.display);
    if banco.lock().unwrap().insert(sig) && enc.fetch_add(1, Ordering::SeqCst) < limite {
        println!("\r🎯 SÉRIE ENCONTRADA: [{} ms]                                   ", ms);
        println!("        ∞\n  ({})  Σ    1 / (n + {})^{} = ALVO\n       n=1\n", a.display.magenta(), b.display.magenta(), k.display.magenta());
    }
}

fn processar_polinomio(coefs: &[&Termo], config: &ConfigBusca, enc: &Arc<AtomicUsize>, banco: &Arc<Mutex<HashSet<String>>>, inicio: Instant) {
    if coefs[0].valor.clone().abs() < 1e-15 { return; }
    let mut res_ok = true;
    for (alvo, prec) in &config.alvos {
        let mut val = Float::with_val(PRECISAO_BITS, 0);
        let grau = coefs.len() - 1;
        for (idx, c) in coefs.iter().enumerate() {
            let p = Float::with_val(PRECISAO_BITS, alvo.clone().pow((grau - idx) as u32));
            val += Float::with_val(PRECISAO_BITS, &c.valor * p);
        }
        if Float::with_val(PRECISAO_BITS, val).abs() > Float::with_val(PRECISAO_BITS, 10.0).pow(-(*prec as i32)) * 1.5 { res_ok = false; break; }
    }
    if res_ok {
        let a_val = &coefs[0].valor;
        let mut sig = String::new();
        for t in coefs { 
            let ratio = Float::with_val(PRECISAO_BITS, t.valor.clone() / a_val);
            sig.push_str(&format!("{:.8}|", ratio)); 
        }
        if banco.lock().unwrap().insert(sig) && enc.fetch_add(1, Ordering::SeqCst) < config.limite {
            renderizar_didatico(coefs, inicio.elapsed().as_millis());
        }
    }
}

fn format_compact(num: u128) -> String {
    if num >= 1_000_000_000_000_000 { format!("{:.2} Quadrilhões", num as f64 / 1_000_000_000_000_000.0) }
    else if num >= 1_000_000_000_000 { format!("{:.2} Trilhões", num as f64 / 1_000_000_000_000.0) }
    else if num >= 1_000_000_000 { format!("{:.2} Bilhões", num as f64 / 1_000_000_000.0) }
    else { format!("{:.2} Milhões", num as f64 / 1_000_000.0) }
}

fn renderizar_didatico(coefs: &[&Termo], ms: u128) {
    let mut lines = [String::new(), String::new(), String::new()];
    let grau = coefs.len() - 1;
    for (i, t) in coefs.iter().enumerate() {
        let exp = grau - i;
        let var = match exp { 0 => "", 1 => "x", 2 => "x²", 3 => "x³", _ => "x^n" };
        let sinal = if t.valor.clone() > 0.0 && i > 0 { "+ " } else if t.valor.clone() < 0.0 { "- " } else { "" };
        let mut n_str = t.display.replace('-', "");
        if n_str == "1" && exp > 0 { n_str = "".into(); }
        let w = n_str.len().max(1);
        lines[0].push_str(&format!("{} {:^w$} {} ", " ".repeat(sinal.len()), n_str, " ".repeat(var.len())));
        lines[1].push_str(&format!("{} {:^w$} {} ", sinal, n_str, var));
        lines[2].push_str(&format!("{} {:^w$} {} ", " ".repeat(sinal.len()), " ", " ".repeat(var.len())));
    }
    println!("\r🎯 IDENTIDADE: [{} ms]                                     \n{}\n{}\n{}\n", ms, lines[0].magenta(), lines[1].magenta(), lines[2].magenta());
}

fn perguntar(m: &str) -> String {
    print!("{}", m.bold().white()); io::stdout().flush().unwrap();
    let mut b = String::new(); io::stdin().read_line(&mut b).unwrap();
    b.trim().into()
}
