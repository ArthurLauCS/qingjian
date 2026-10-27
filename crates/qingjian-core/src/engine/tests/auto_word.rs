//! 自动造词：一段拼音上屏完选一次就造、撤销时删掉、简拼也打得出来；分开选的词下次整句连着出。

use super::*;

/// 测试从外面看学习器造了哪些词。
type Learned = Arc<Mutex<(Vec<String>, sentence::UserNgram)>>;

fn word_engine() -> (Engine, Learned) {
    let shared = Arc::new(Mutex::new((Vec::new(), sentence::UserNgram::default())));
    let learner = WordLearner {
        shared: Arc::clone(&shared),
        ..WordLearner::default()
    };
    (engine().with_learner(Box::new(learner)), shared)
}

fn commit_first(engine: &mut Engine, input: &str) -> Candidate {
    engine.set_input(input);
    let first = engine.query().unwrap().candidates.items[0].clone();
    engine.commit(&first);
    first
}

fn pick(engine: &mut Engine, input: &str, text: &str) {
    engine.set_input(input);
    let candidate = engine
        .query()
        .unwrap()
        .candidates
        .items
        .into_iter()
        .find(|c| c.text == text && c.kind == CandidateKind::Chinese)
        .unwrap();
    engine.commit(&candidate);
}

#[test]
fn a_sentence_of_single_characters_becomes_a_word_at_once() {
    let (mut engine, shared) = word_engine();
    // 整句里有词库词（想 + 开发）是拼出来的短句，不造
    let sentence = commit_first(&mut engine, "xiangkaifa");
    assert_eq!(sentence.text, "想开发");
    assert!(shared.lock().unwrap().0.is_empty());
    engine.punctuate('。');
    // 全是单字（想 + 先）：选一次就造，不记「整段拼音 → 句子」的选择
    let sentence = commit_first(&mut engine, "xiangxian");
    assert_eq!(
        (sentence.kind, sentence.text.as_str()),
        (CandidateKind::Sentence, "想先")
    );
    assert_eq!(shared.lock().unwrap().0, ["想先"]);
    assert_eq!(engine.learner().choice_weight("xiangxian", "想先"), 0);
    engine.punctuate('。');
    engine.set_input("xx");
    assert_eq!(texts_of(&engine)[0], "想先");
}

#[test]
fn erasing_a_coined_word_and_choosing_another_forgets_it() {
    let (mut engine, shared) = word_engine();
    commit_first(&mut engine, "xiangxian");
    assert_eq!(shared.lock().unwrap().0, ["想先"]);
    engine.note_backspace();
    engine.note_backspace();
    pick(&mut engine, "xiangxian", "想");
    assert!(shared.lock().unwrap().0.is_empty());
}

#[test]
fn words_picked_one_after_another_lead_the_sentence_next_time() {
    let dictionary =
        Dictionary::parse("俺\tan\t3000\n按\tan\t500\n推荐\ttui jian\t5000\n").unwrap();
    let mut engine = Engine::new(dictionary).with_learner(Box::new(WordLearner::default()));
    engine.set_input("antuijian");
    assert_eq!(texts_of(&engine)[0], "俺推荐");
    // 分开打 an 选 按、tuijian 选 推荐，一次之后整段打出来就是 按推荐
    pick(&mut engine, "an", "按");
    pick(&mut engine, "tuijian", "推荐");
    engine.punctuate('。');
    engine.set_input("antuijian");
    assert_eq!(texts_of(&engine)[0], "按推荐");
}
