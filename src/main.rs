use clap::Parser;
use rug::{Float, float::Constant, ops::Pow};
use std::io::{self, Write, BufWriter};
use std::fs::OpenOptions;
use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rayon::prelude::*;
use std::collections::HashSet;
use colored::*;
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use crossterm::event::{self, Event, KeyCode};

const PRECISAO_BITS: u32 = 128;

#[derive(Parser, Debug)]
#[command(name = "Keshet", author = "Tiago Rabelo (HonoravelMacho)", version = "1.0.1")]
struct Args {
    #[arg(short = 'g', long)] grau: Option<usize>,
    #[arg(short = 'a', long)] alcance: Option<i64>,
    #[arg(short = 'r', long)] resultados: Option<usize>,
    #[arg(short = 'n', long, num_args = 1.., allow_hyphen_values = true)] numeros: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
struct Termo {
    valor: Float,
    display_n: String,
    display_d: Option<String>,
    complexidade: u8,
}

struct ConfigBusca {
    grau: usize,
    alcance: i64,
    limite: usize,
    alvos: Vec<(Float, usize)>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(n_flags) = args.numeros {
        let config = validar_flags_direto(n_flags, args.grau, args.alcance, args.resultados)?;
        executar_busca(config)?;
    } else {
        menu_principal()?;
    }
    Ok(())
}

fn menu_principal() -> anyhow::Result<()> {
    loop {
        println!("\n{}", "=================================================".cyan());
        println!("   {} - The Master Alchemist v1.0.1", "קֶשֶׁת (KESHET)".bold().yellow());
        println!("{}", "=================================================".cyan());
        println!(" 1. 🔍 Iniciar Alquimia Profunda (Somas e Composições)");
        println!(" 2. 📖 Manual das Flags (Uso via Terminal)");
        println!(" 0. 🚪 Sair");
        print!("\nCapitão, sua ordem: ");
        io::stdout().flush()?;
        let mut opt = String::new();
        io::stdin().read_line(&mut opt)?;
        match opt.trim() {
            "1" => { configurar_interativo()?; },
            "2" => { mostrar_ajuda(); },
            "0" => { std::process::exit(0); },
            _ => { println!("{}", "Opção inválida!".red()); }
        }
    }
}

fn mostrar_ajuda() {
    println!("\n{}", "--- MANUAL DE OPERAÇÕES DO KESHET ---".yellow().bold());
    println!("Você pode usar o Keshet diretamente pelo terminal com flags:");
    println!("Exemplo: {}","keshet -g 2 -a 10 -n 1.90216".green());
    println!("\nFlags disponíveis:");
    println!("  -g <int> : Define o Grau do polinômio (Padrão: 2).");
    println!("  -a <int> : Alcance dos coeficientes (Inteiros base para as fórmulas).");
    println!("  -r <int> : Limite de resultados antes de encerrar.");
    println!("  -n <vals>: Lista de constantes alvo (Pode usar várias).");
    println!("\n{}", "Controles durante a busca:".white().bold());
    println!("  [ESPAÇO] ou [P] para pausar/retomar a caçada.");
}

// ... (Mantenha todas as outras funções: configurar_interativo, gerar_universo_alquimico, executar_busca, etc., exatamente como na v8.5)

fn configurar_interativo() -> anyhow::Result<()> {
    let grau: usize = perguntar("Grau (2 ou 3): ").parse().unwrap_or(2);
    let alcance: i64 = perguntar("Alcance Inteiro (A): ").parse().unwrap_or(5);
    let limite: usize = perguntar("Resultados: ").parse().unwrap_or(10);
    
    let mut alvos = Vec::new();
    println!("Dica: Use ponto ou vírgula. ENTER para pular raízes extras.");
    for i in 1..=grau {
        let mut input = perguntar(&format!("Constante {}: ", i));
        if input.is_empty() { continue; }
        input = input.replace(',', ".");
        let prec = input.split('.').nth(1).map_or(0, |f| f.len());
        alvos.push((Float::with_val(PRECISAO_BITS, Float::parse(&input).expect("Erro")), prec));
    }
    executar_busca(ConfigBusca { grau, alcance, limite, alvos })
}

fn gerar_universo_alquimico(alcance: i64) -> Vec<Termo> {
    let mut u = Vec::new();
    let bits = PRECISAO_BITS;
    let mut bases = Vec::new();
    bases.push((Float::with_val(bits, Constant::Pi), "π".into(), 2));
    bases.push((Float::with_val(bits, 1.0).exp(), "e".into(), 2));
    bases.push(((Float::with_val(bits, 5.0).sqrt() + 1.0) / 2.0, "Φ".into(), 2));
    bases.push((Float::with_val(bits, Constant::Log2), "ln2".into(), 2));
    bases.push((Float::with_val(bits, Constant::Catalan), "G".into(), 3));
    bases.push((Float::with_val(bits, Constant::Euler), "γ".into(), 3));
    for n in 1..=alcance {
        bases.push((Float::with_val(bits, n), n.to_string(), 1));
        if (n as f64).sqrt() % 1.0 != 0.0 {
            bases.push((Float::with_val(bits, n).sqrt(), format!("√{}", n), 3));
        }
        bases.push((Float::with_val(bits, n as f64).ln(), format!("ln({})", n), 4));
    }
    let mut compostos = Vec::new();
    let mut vistos_valores = HashSet::new();
    for (v1, s1, c1) in &bases {
        for (v2, s2, c2) in &bases {
            if s1 != s2 && (*c1 >= 2 || *c2 >= 2) {
                compostos.push(Termo {
                    valor: Float::with_val(bits, v1 / v2),
                    display_n: s1.clone(),
                    display_d: Some(s2.clone()),
                    complexidade: c1 + c2,
                });
            }
            if *c1 >= 2 && *c2 == 1 {
                compostos.push(Termo {
                    valor: Float::with_val(bits, v1 + v2),
                    display_n: format!("{}+{}", s1, s2),
                    display_d: None,
                    complexidade: c1 + 1,
                });
                compostos.push(Termo {
                    valor: Float::with_val(bits, v1 - v2),
                    display_n: format!("{}-{}", s1, s2),
                    display_d: None,
                    complexidade: c1 + 1,
                });
            }
        }
    }
    for (v, s, c) in bases {
        u.push(Termo { valor: v.clone(), display_n: s.clone(), display_d: None, complexidade: c });
        u.push(Termo { valor: Float::with_val(bits, -v), display_n: format!("-{}", s), display_d: None, complexidade: c });
    }
    for comp in compostos {
        let key = format!("{:.12}", comp.valor);
        if !vistos_valores.contains(&key) {
            vistos_valores.insert(key);
            u.push(comp);
        }
    }
    u.push(Termo { valor: Float::with_val(bits, 0), display_n: "0".into(), display_d: None, complexidade: 1 });
    u.sort_by(|a, b| b.complexidade.cmp(&a.complexidade));
    u
}

fn executar_busca(config: ConfigBusca) -> anyhow::Result<()> {
    let universo = gerar_universo_alquimico(config.alcance);
    let total_perm = (universo.len() as u128).pow((config.grau + 1) as u32);
    let encontrados = Arc::new(AtomicUsize::new(0));
    let pausado = Arc::new(AtomicBool::new(false));
    let p_clone = Arc::clone(&pausado);
    let banco_textual = Arc::new(Mutex::new(HashSet::new()));
    println!("\n🚀 {} combinações. FOGO!", format_compact(total_perm).magenta().bold());
    let pb = ProgressBar::new(universo.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} (ETA: {eta})").unwrap().progress_chars("█▉▊▋▌▍▎▏"));
    std::thread::spawn(move || {
        loop {
            if event::poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if key.code == KeyCode::Char(' ') || key.code == KeyCode::Char('p') {
                        let estado = p_clone.fetch_xor(true, Ordering::SeqCst);
                        if !estado { println!(" {}", "⏸ PAUSADO".yellow().bold()); }
                        else { println!(" {}", "▶ RETOMANDO...".green().bold()); }
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
                if config.grau == 2 {
                    let c = [&universo[i], &universo[j], &universo[k]];
                    processar_candidato(&c, &config, &encontrados, &banco_textual, inicio);
                } else {
                    for &l in &idx {
                        let c = [&universo[i], &universo[j], &universo[k], &universo[l]];
                        processar_candidato(&c, &config, &encontrados, &banco_textual, inicio);
                    }
                }
            }
        }
    });
    pb.finish_with_message("Concluído");
    println!("\n✅ Relatório salvo em 'keshet_descobertas.txt'.");
    Ok(())
}

fn processar_candidato(coefs: &[&Termo], config: &ConfigBusca, encontrados: &Arc<AtomicUsize>, banco: &Arc<Mutex<HashSet<String>>>, inicio: Instant) {
    if coefs[0].valor == 0 { return; }
    if validar(coefs, &config.alvos) {
        let a_val = &coefs[0].valor;
        let sig_b = Float::with_val(PRECISAO_BITS, &coefs[1].valor / a_val);
        let sig_c = if coefs.len() > 2 { Float::with_val(PRECISAO_BITS, &coefs[2].valor / a_val) } else { Float::with_val(PRECISAO_BITS, 0) };
        let sig = format!("{:.8}_{:.8}", sig_b, sig_c);
        let mut b = banco.lock().unwrap();
        if !b.contains(&sig) {
            b.insert(sig);
            if encontrados.fetch_add(1, Ordering::SeqCst) < config.limite {
                renderizar_didatico(coefs, inicio.elapsed().as_millis());
                let _ = gravar_em_arquivo(coefs, inicio.elapsed().as_millis());
            }
        }
    }
}

fn format_compact(num: u128) -> String {
    if num >= 1_000_000_000_000 { format!("{:.2} Trilhões", num as f64 / 1_000_000_000_000.0) }
    else if num >= 1_000_000_000 { format!("{:.2} Bilhões", num as f64 / 1_000_000_000.0) }
    else if num >= 1_000_000 { format!("{:.2} Milhões", num as f64 / 1_000_000.0) }
    else { num.to_string() }
}

fn validar(coefs: &[&Termo], alvos: &[(Float, usize)]) -> bool {
    for (alvo, prec) in alvos {
        let mut res = Float::with_val(PRECISAO_BITS, 0);
        let grau = coefs.len() - 1;
        for (i, c) in coefs.iter().enumerate() {
            res += Float::with_val(PRECISAO_BITS, &c.valor * alvo.clone().pow((grau-i) as u32));
        }
        let margem = Float::with_val(PRECISAO_BITS, 10.0).pow(-(*prec as i32)) * 1.5;
        if res.abs() > margem { return false; }
    }
    true
}

fn renderizar_didatico(coefs: &[&Termo], ms: u128) {
    let mut top = String::new(); let mut mid = String::new(); let mut bot = String::new();
    let grau = coefs.len() - 1;
    for (i, t) in coefs.iter().enumerate() {
        let exp = grau - i;
        let var = match exp { 0 => "", 1 => "x", 2 => "x²", 3 => "x³", _ => "x^n" };
        let sinal = if t.valor > 0 && i > 0 { "+ " } else if t.valor < 0 { "- " } else { "" };
        let mut n_str = t.display_n.replace('-', "");
        let d_str = t.display_d.clone().unwrap_or_else(|| "".into());
        if n_str == "1" && exp > 0 && d_str.is_empty() { n_str = "".into(); }
        let width = n_str.len().max(d_str.len()).max(1);
        top.push_str(&format!("{} {:^width$}  {} ", " ".repeat(sinal.len()), n_str, " ".repeat(var.len())));
        if d_str.is_empty() {
            mid.push_str(&format!("{} {:^width$} {} ", sinal, n_str, var));
            bot.push_str(&format!("{} {:^width$}  {} ", " ".repeat(sinal.len()), " ", " ".repeat(var.len())));
            let start = top.len() - width - var.len() - sinal.len() - 2;
            top.replace_range(start.., &" ".repeat(width + var.len() + sinal.len() + 2));
        } else {
            mid.push_str(&format!("{} {:─^width$}  {} ", sinal, "─", var));
            bot.push_str(&format!("{} {:^width$}  {} ", " ".repeat(sinal.len()), d_str, " ".repeat(var.len())));
        }
    }
    println!("\n🎯 [{} ms]\n{}\n{}\n{}", ms, top.magenta(), mid.magenta(), bot.magenta());
}

fn gravar_em_arquivo(coefs: &[&Termo], ms: u128) -> io::Result<()> {
    let mut f = OpenOptions::new().append(true).open("keshet_descobertas.txt")?;
    let mut line = format!("[{:>10} ms] ", ms);
    for (i, t) in coefs.iter().enumerate() {
        let var = match coefs.len() - 1 - i { 0 => "", 1 => "x", 2 => "x²", 3 => "x³", _ => "x^n" };
        line.push_str(&format!("({}/{}){} ", t.display_n, t.display_d.as_ref().unwrap_or(&"1".to_string()), var));
    }
    writeln!(f, "{} = 0", line)
}

fn perguntar(m: &str) -> String {
    print!("{}", m.bold().white()); io::stdout().flush().unwrap();
    let mut b = String::new(); io::stdin().read_line(&mut b).unwrap();
    b.trim().into()
}

fn validar_flags_direto(numeros: Vec<String>, grau: Option<usize>, alcance: Option<i64>, limite: Option<usize>) -> anyhow::Result<ConfigBusca> {
    let alvos = numeros.iter().map(|s| {
        let s_fixed = s.replace(',', ".");
        (Float::with_val(PRECISAO_BITS, Float::parse(&s_fixed).unwrap()), s_fixed.split('.').nth(1).map_or(0, |f| f.len()))
    }).collect();
    Ok(ConfigBusca { grau: grau.unwrap_or(2), alcance: alcance.unwrap_or(5), limite: limite.unwrap_or(10), alvos })
}
