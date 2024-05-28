use crate::equation_generator::*;
use cursive::event::Key;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};
use cursive::{Cursive, CursiveExt, With};

mod equation_generator;

fn main() {
    let mut ctx = Cursive::default();
    ctx.set_fps(30);
    ctx.add_global_callback(Key::Esc, |s| s.quit());

    settings_screen(&mut ctx);
    ctx.run();
}

fn settings_screen(ctx: &mut Cursive) {
    ctx.pop_layer();

    let settings_dialog = Dialog::new()
        .content(
            ListView::new()
                .child(
                    "Максимальное число",
                    EditView::default().with_name("max_number").min_width(10),
                )
                .child(
                    "Сложение/Вычитание",
                    Checkbox::new().checked().with_name("include_add_sub"),
                )
                .child("Умножение", Checkbox::new().with_name("include_mul"))
                .child("Деление", Checkbox::new().with_name("include_div"))
                .child("Уравнения", Checkbox::new().with_name("hide_operand"))
                .child(
                    "Отрицательные числа",
                    Checkbox::new().with_name("can_be_negative"),
                ),
        )
        .button("Погнали!", game_screen);

    ctx.add_layer(Dialog::new().content(settings_dialog));
    ctx.call_on_name("max_number", |x: &mut EditView| x.set_content("100"));
}

fn game_screen(ctx: &mut Cursive) {
    let settings = generate_settings(ctx);
    let mut eq_vec = Vec::new();

    for _ in 0..20 {
        eq_vec.push(Equation::new(&settings));
    }

    let list_view = ListView::new()
        .child("Пример", TextView::new("Твой ответ"))
        .with(|list| {
            for (i, x) in eq_vec.iter().enumerate() {
                list.add_child(
                    &x.string_representation,
                    EditView::new().with_name(format!("eq_{}", i)).min_width(8),
                );
            }
        });

    ctx.pop_layer();
    ctx.add_layer(
        Dialog::new()
            .content(list_view)
            .button("Готово", move |s| score_screen(s, &eq_vec)),
    )
}

fn score_screen(ctx: &mut Cursive, eq_vec: &[Equation]) {
    let mut score = 0;

    for (i, eq) in eq_vec.iter().enumerate() {
        let answer_string =
            ctx.call_on_name(&format!("eq_{}", i), |x: &mut EditView| x.get_content());

        let answer_string = match answer_string {
            Some(string) => string.to_string(),
            _ => String::from("-"),
        };

        if answer_string == "-" {
            continue;
        };

        let answer_int: i32 = match answer_string.parse() {
            Ok(x) => x,
            _ => continue,
        };

        if eq.check_answer(answer_int) {
            score += 1;
        }
    }

    let end_result = format!(
        "Вы ответили правильно на {}/{} уравнений...",
        score,
        eq_vec.len()
    );

    ctx.pop_layer();
    ctx.add_layer(
        Dialog::around(TextView::new(end_result)).button("В главное меню", settings_screen),
    )
}

pub fn generate_settings(ctx: &mut Cursive) -> Settings {
    let max_number_temp_string = ctx.call_on_name("max_number", |x: &mut EditView| x.get_content());

    let string = match max_number_temp_string {
        Some(string) => string.to_string(),
        _ => String::from("100"),
    };

    let mut max_number: i32 = match string.parse() {
        Ok(x) => x,
        _ => 100,
    };

    if max_number <= 0 {
        max_number = 100;
    }

    let include_add_sub = ctx
        .call_on_name("include_add_sub", |x: &mut Checkbox| x.is_checked())
        .unwrap();
    let include_mul = ctx
        .call_on_name("include_mul", |x: &mut Checkbox| x.is_checked())
        .unwrap();
    let include_div = ctx
        .call_on_name("include_div", |x: &mut Checkbox| x.is_checked())
        .unwrap();
    let hide_operand = ctx
        .call_on_name("hide_operand", |x: &mut Checkbox| x.is_checked())
        .unwrap();
    let can_be_negative = ctx
        .call_on_name("can_be_negative", |x: &mut Checkbox| x.is_checked())
        .unwrap();

    Settings::new(
        max_number,
        include_add_sub,
        include_mul,
        include_div,
        hide_operand,
        can_be_negative,
    )
}
