use leptonic::prelude::*;
use leptos::*;



#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ObjectStat {
    id: u8,
    name: String,
    term: u16,
    cash: u16,
    shampoo: String,
    foam: String,
    wax: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Statistic {
    date: String,
    data: Vec<ObjectStat>,
}
impl Statistic {
    fn new() -> Self {
        Statistic {
            date: String::new(),
            data: Vec::new()
        }
    }
}

#[component]
pub fn Page() -> impl IntoView {
    let obj1 = ObjectStat {
        id: 8,
        name: "Брн.Челюскинцев".to_string(),
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
    let mut StatList = Statistic::new();
    StatList.date = "27.02.2024".to_string();
    StatList.data.push(obj1);
    StatList.data.push(obj2);
    let (stat_list, _set_stat_list) = create_signal(StatList);

    view! {
        <Box class="main-container">
            <Box class="card">
                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/icupken/manager_web/main/logo%20(1).png"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/icupken/manager_web/main/logo%20(1).png"
                        alt="Leptos Logo"
                        height="150"
                        width="150"
                    />
                </picture>

                <p>{stat_list.get().date}</p>

                <TableContainer style = "--table-body-cell-padding: 20px">
                    <Table bordered=true hoverable=true>
                        <TableHeader>
                            <TableRow>
                                // <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                                <TableHeaderCell>"Имя"</TableHeaderCell>
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
                                    key=|n| n.id
                                    children=move |data: ObjectStat| {
                                        view! {
                                            <TableRow>
                                            <TableCell>{data.name}</TableCell>
                                            <TableCell>{data.term}</TableCell>
                                            <TableCell>{data.cash}</TableCell>
                                            <TableCell>{data.cash+data.term}</TableCell>
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

            </Box>
        </Box>
    }
}
