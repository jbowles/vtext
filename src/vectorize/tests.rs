// Copyright 2019 vtext developers
//
// Licensed under the Apache License, Version 2.0,
// <http://apache.org/licenses/LICENSE-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use crate::tokenize::*;
use crate::vectorize::*;

#[test]
fn test_count_vectorizer_simple() {
    // Example 1

    let documents = vec!["cat dog cat".to_string()];
    let mut vect = CountVectorizer::<RegexpTokenizer>::default();

    let X = vect.fit_transform(&documents);
    assert_eq!(X.to_dense(), array![[2, 1]]);

    // Example 1
    let documents = vec![
        "the moon in the sky".to_string(),
        "The sky sky sky is blue".to_string(),
    ];
    let X_ref = array![[0, 1, 0, 1, 1, 2], [1, 0, 1, 0, 3, 1]];
    let mut vect = CountVectorizer::<RegexpTokenizer>::default();

    let X = vect.fit_transform(&documents);
    assert_eq!(X.to_dense().shape(), X_ref.shape());
    assert_eq!(X.to_dense(), X_ref);

    vect.fit(&documents);
    let X = vect.transform(&documents);
    assert_eq!(X.to_dense().shape(), X_ref.shape());
    assert_eq!(X.to_dense(), X_ref);
}

#[test]
fn test_vectorize_empty_countvectorizer() {
    let documents = vec!["some tokens".to_string(), "".to_string()];

    let mut vect = CountVectorizer::<RegexpTokenizer>::default();
    vect.fit_transform(&documents);

    vect.fit(&documents);
    vect.transform(&documents);
}

#[test]
fn test_vectorize_empty_hashingvectorizer() {
    let documents = vec!["some tokens".to_string(), "".to_string()];

    let vect = HashingVectorizer::<RegexpTokenizer>::default();
    vect.fit_transform(&documents);

    vect.transform(&documents);
}

#[test]
fn test_count_vectorizer_fit_transform() {
    for documents in &[vec!["cat dog cat".to_string()]] {
        let mut vect = CountVectorizer::<RegexpTokenizer>::default();
        vect.fit(&documents);
        let X = vect.transform(&documents);

        let mut vect2 = CountVectorizer::<RegexpTokenizer>::default();
        let X2 = vect2.fit_transform(&documents);
        assert_eq!(vect.vocabulary, vect2.vocabulary);
        println!("{:?}", vect.vocabulary);
        assert_eq!(X.to_dense(), X2.to_dense());
    }
}

#[test]
fn test_hashing_vectorizer_simple() {
    // Results with scikit-learn 0.20.0
    // >>> vect = HashingVectorizer(norm=None, alternate_sign=False)
    // >>> X = vect.fit_transform(['the moon in the sky', 'The sky is blue'])
    // >>> X.indices
    // array([268391, 286878, 720286, 828689, 144749, 268391, 286878, 790269],
    //       dtype=int32)
    // >>> X.indptr
    // array([0, 4, 8], dtype=int32)
    // >>> X.data
    // array([1., 2., 1., 1., 1., 1., 1., 1.])
    let documents = vec![
        String::from("the moon in the sky"),
        String::from("The sky is blue"),
    ];

    let vect = HashingVectorizer::<VTextTokenizer>::default();
    let vect = vect.fit(&documents);
    let X = vect.transform(&documents);
    assert_eq!(X.indptr(), &[0, 4, 8]);
    assert_eq!(X.data(), &[1, 2, 1, 1, 1, 1, 1, 1]);
    // this is not a thorough test because indices don't match exactly
    // as hashing is not exactly identical
    assert_eq!(X.data().len(), X.indices().len());

    let mut indices_ref = vec![
        268391, 286878, 720286, 828689, 144749, 268391, 286878, 790269,
    ];
    indices_ref.sort();
    indices_ref.dedup();
    let mut indices = X.indices().to_vec();
    indices.sort();
    indices.dedup();
    assert_eq!(indices_ref.len(), indices.len());

    let X2 = vect.fit_transform(&documents);
    //assert_eq!(X.indices, X2.indices);
    assert_eq!(X.indptr(), X2.indptr());
    assert_eq!(X.data(), X2.data());
}

#[test]
fn test_empty_dataset() {
    let documents: Vec<String> = vec![];

    let tokenizer = VTextTokenizerParams::default().build().unwrap();
    let mut vectorizer = CountVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();

    let X = vectorizer.fit_transform(&documents);
    assert_eq!(X.data(), &[]);
    assert_eq!(X.indices(), &[]);
    assert_eq!(X.indptr(), &[0]);

    let vectorizer = HashingVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();

    let X = vectorizer.fit_transform(&documents);
    assert_eq!(X.data(), &[]);
    assert_eq!(X.indices(), &[]);
    assert_eq!(X.indptr(), &[0]);
}

#[test]
fn test_dispatch_tokenizer() {
    let tokenizer = VTextTokenizerParams::default().build().unwrap();
    CountVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();
    HashingVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();

    let tokenizer = UnicodeSegmentTokenizerParams::default()
        .word_bounds(false)
        .build()
        .unwrap();
    CountVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();
    HashingVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();

    let tokenizer = RegexpTokenizerParams::default().build().unwrap();
    CountVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();
    HashingVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();

    let tokenizer = CharacterTokenizerParams::default()
        .window_size(3)
        .build()
        .unwrap();
    CountVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();
    HashingVectorizerParams::default()
        .tokenizer(tokenizer.clone())
        .build()
        .unwrap();
}

#[test]
#[cfg(feature = "rayon")]
fn test_vectorizers_n_jobs() {
    let documents = vec![
        String::from("the moon in the sky"),
        String::from("the sky is blue"),
        String::from("some other text"),
        String::from("other words"),
    ];

    let mut vect = CountVectorizerParams::<RegexpTokenizer>::default()
        .n_jobs(2)
        .build()
        .unwrap();
    assert_eq!(vect.params.n_jobs, 2);
    let X = vect.fit(&documents);
}
