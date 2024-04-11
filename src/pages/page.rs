use leptonic::prelude::*;
use leptos::*;
use time::OffsetDateTime;
use tracing::info;
    use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize ,Clone, Debug, PartialEq, Eq, Hash)]
struct ObjectStat {
    id: u8,
    name: String,
    term: u16,
    cash: u16,
    shampoo: String,
    foam: String,
    wax: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct Statistic {
    data: Vec<ObjectStat>,
}
impl Statistic {
    fn new() -> Self {
        Statistic {
            data: Vec::new()
        }
    }
}

#[component]
pub fn Page() -> impl IntoView {
    let obj1 =  ObjectStat {
        id: 17,
        name: "Брн.Павловский".to_string(),
        term: 14960,
        cash: 10312,
        shampoo: "2.7".to_string(),
        foam: "3.1".to_string(),
        wax: "0.4".to_string(),
    };
    let obj2 = ObjectStat {
        id: 17,
        name: "Брн.Павловский".to_string(),
        term: 14960,
        cash: 10312,
        shampoo: "2.7".to_string(),
        foam: "3.1".to_string(),
        wax: "0.4".to_string(),
    };
    let obj3 = ObjectStat {
        id: 17,
        name: "Брн.Павловский".to_string(),
        term: 14960,
        cash: 10312,
        shampoo: "2.7".to_string(),
        foam: "3.1".to_string(),
        wax: "0.4".to_string(),
    };
    let obj4 = ObjectStat {
        id: 17,
        name: "Брн.Павловский".to_string(),
        term: 14960,
        cash: 10312,
        shampoo: "2.7".to_string(),
        foam: "3.1".to_string(),
        wax: "0.4".to_string(),
    };

    let mut StatList = Statistic::new();
    StatList.data.push(obj1);
    StatList.data.push(obj2);
    StatList.data.push(obj3);
    StatList.data.push(obj4);
    let (stat_list, set_stat_list) = create_signal(StatList);

    view! {
        <Box class="main-container">
            <Box class="card">
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Px(90)>
                    <Box style="background: transparent; margin-left: 50px">
                        <picture>
                            <source
                                srcset="https://raw.githubusercontent.com/icupken/manager_web/main/logo%20(1).png"
                                media="(prefers-color-scheme: dark)"
                            />
                            <img
                                src="https://raw.githubusercontent.com/icupken/manager_web/main/logo%20(1).png"
                                alt="Leptos Logo"
                                height="210"
                                width="210"
                            />
                        </picture>
                    </Box>
                    <DateSelector
                        value=OffsetDateTime::now_utc()
                        on_change=move |v| {
                            let obj5 = ObjectStat {
                                id: 17,
                                name: "Брн.Павловский".to_string(),
                                term: 14960,
                                cash: 10312,
                                shampoo: "2.7".to_string(),
                                foam: "3.1".to_string(),
                                wax: "0.4".to_string(),
                            };
                            set_stat_list.update(|f| f.data.push(obj5));
                            info!("{:?}", v);
                     }
                    />
                </Stack>
                <TableContainer style="--table-body-cell-padding: 20px">
                    <Table bordered=true hoverable=true>
                        <TableHeader>
                            <TableRow>
                                // <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                                <TableHeaderCell>"Объект"</TableHeaderCell>
                                <TableHeaderCell>"Терминал"</TableHeaderCell>
                                <TableHeaderCell>"Карты"</TableHeaderCell>
                                <TableHeaderCell>"Итого"</TableHeaderCell>
                                <TableHeaderCell>"Шампунь"</TableHeaderCell>
                                <TableHeaderCell>"Пена"</TableHeaderCell>
                                <TableHeaderCell>"Воск"</TableHeaderCell>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <For
                                each=move || stat_list.get().data
                                key=|data| data.id
                                children=move |data: ObjectStat| {
                                    view! {
                                        <TableRow>
                                            <TableCell>{data.name}</TableCell>
                                            <TableCell>{data.term}</TableCell>
                                            <TableCell>{data.cash}</TableCell>
                                            <TableCell>{data.cash + data.term}</TableCell>
                                            <TableCell>{data.shampoo}</TableCell>
                                            <TableCell>{data.foam}</TableCell>
                                            <TableCell>{data.wax}</TableCell>
                                        </TableRow>
                                    }
                                }
                            />

                        </TableBody>
                    </Table>
                </TableContainer>
                <div class="no_data">
                    <p>
                        {move || {
                            if stat_list.get().data.is_empty() { "Нет данных" } else { "" }
                        }}

                    </p>
                </div>
            </Box>
        </Box>
    }
}
