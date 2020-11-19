use crate::{
    database::*,
    errors::{ServiceError, ServiceResult},
};
use diesel::prelude::*;
use actix::prelude::*;
use actix_web::web;
use actix_identity::Identity;

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct RelatedProblem {
    pub region: String,
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct TestCaseCatalogElement {
    pub name: String,
    pub is_spj: bool,
    pub count: i32,
    pub related_problems: Vec<RelatedProblem>,
}

#[derive(Debug, Clone, Serialize, juniper::GraphQLObject)]
pub struct TestCaseCatalog {
    pub total_count: i32,
    pub elements: Vec<Vec<TestCaseCatalogElement>>,
    pub page_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTestCaseCatalogMessage {
    pub elements_per_page: Option<i32>,
}

impl Message for GetTestCaseCatalogMessage {
    type Result = Result<TestCaseCatalog, String>;
}

impl Handler<GetTestCaseCatalogMessage> for DbExecutor {
    type Result = Result<TestCaseCatalog, String>;
    
    fn handle(&mut self, msg: GetTestCaseCatalogMessage, _: &mut Self::Context) -> Self::Result {
        use crate::schema::test_cases;
        use crate::schema::problems;
        use crate::test_case::model::*;

        let result = test_cases::table
            .load::<TestCase>(&self.0)
            .expect("Error loading testcases.");
        
        let mut catalog = TestCaseCatalog {
            total_count: 0,
            elements: Vec::new(),
            page_count: 0,
        };
        let mut current_page_number = 0;
        let mut page_element_count = 0;
        catalog.elements.push(Vec::new());
        for test_case in result {
            if !msg.elements_per_page.is_none() && msg.elements_per_page.unwrap() == page_element_count {
                catalog.elements.push(Vec::new());
                current_page_number += 1;
                page_element_count = 0;
            }
            catalog.elements[current_page_number as usize].push(
                TestCaseCatalogElement {
                    name: test_case.name.clone(),
                    is_spj: test_case.is_spj,
                    count: test_case.count,
                    related_problems: {
                        let result = problems::table.filter(problems::test_case.nullable().eq(test_case.name))
                            .select((problems::region, problems::id, problems::title))
                            .load::<(String, i32, String)>(&self.0)
                            .expect("Error while loading problems");

                        let mut out = Vec::new();
                        for (region, id, title) in result {
                            out.push(RelatedProblem {
                                region: region,
                                id: id,
                                title: title,
                            });
                        }
                        
                        out
                    },
                }
            );
            page_element_count += 1;
            catalog.total_count += 1;
        }
        
        catalog.total_count += 1;
        Ok(catalog)
    }
}

pub async fn get_test_case_catalog_service(
    data: web::Data<DBState>,
    elements_per_page: Option<i32>,
    _id: Identity,
) -> ServiceResult<TestCaseCatalog> {
    let db_result = data.db.send(GetTestCaseCatalogMessage {
        elements_per_page: elements_per_page,
    }).await;

    match db_result {
        Err(_) => Err(ServiceError::InternalServerError),
        Ok(inner_result) => {
            match inner_result {
                Err(msg) => Err(ServiceError::BadRequest(msg)),
                Ok(catalog) => Ok(catalog),
            }
        }
    }
}